package api

import (
	"errors"
	"io"
	"io/fs"
	"log"
	"net"
	"os"
	"strings"

	generateSql "github.com/DeveloperAromal/SynapseDB/ai/v1"
	executor "github.com/DeveloperAromal/SynapseDB/api/execute"
	"github.com/joho/godotenv"
)

// The TCP protocol has no authentication, so the server only listens on the
// loopback interface and cannot be reached from other machines.
const listenAddr = "127.0.0.1:4538"

// parseErrorPrefix is how the Rust engine reports SQL it could not parse.
const parseErrorPrefix = "Parse error:"

var sqlKeywords = []string{"SELECT", "INSERT", "CREATE"}

func Start(ready chan<- struct{}) {
	// .env is optional: the variables can also come from the real environment.
	if err := godotenv.Load(); err != nil && !errors.Is(err, fs.ErrNotExist) {
		log.Printf("Failed to load .env: %v", err)
	}

	ln, err := net.Listen("tcp", listenAddr)
	if err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
	if ready != nil {
		close(ready)
	}

	defer ln.Close()

	for {
		conn, err := ln.Accept()
		if err != nil {
			log.Printf("Failed to accept connection: %v", err)
			continue
		}
		go handleConnection(conn)
	}
}

func handleConnection(conn net.Conn) {
	defer conn.Close()

	buf := make([]byte, 1024)
	n, err := conn.Read(buf)
	if err != nil {
		// io.EOF is a client that connected and left without sending a query,
		// such as the shell checking that the server is up.
		if !errors.Is(err, io.EOF) {
			log.Printf("Error reading: %v", err)
		}
		return
	}

	result := processQuery(strings.TrimSpace(string(buf[:n])))

	_, err = conn.Write([]byte(result + "\n"))
	if err != nil {
		log.Printf("Error writing to client: %v", err)
	}
}

// processQuery runs input that is already SQL directly on the engine and
// sends everything else through the natural language to SQL model.
func processQuery(raw string) string {
	if raw == "" {
		return "Error: empty query"
	}

	apiKey := os.Getenv("OPENROUTER_API_KEY")

	if looksLikeSQL(raw) {
		result := executor.Execute(raw)
		// A parse error can mean the input was English that happens to start
		// with an SQL keyword ("select users older than 30"), so let the model
		// try it when one is configured.
		if !strings.HasPrefix(result, parseErrorPrefix) || apiKey == "" {
			return result
		}
	}

	query, err := generateSql.GenerateSQL(raw, apiKey, os.Getenv("MODEL"))
	if err != nil {
		log.Printf("SQL generation failed: %v", err)
		return "Error: " + err.Error()
	}

	return executor.Execute(query)
}

func looksLikeSQL(input string) bool {
	fields := strings.Fields(input)
	if len(fields) == 0 {
		return false
	}

	for _, keyword := range sqlKeywords {
		if strings.EqualFold(fields[0], keyword) {
			return true
		}
	}
	return false
}
