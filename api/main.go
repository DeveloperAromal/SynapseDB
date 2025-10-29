package api

import (
	"log"
	"net"
	"os"

	generateSql "github.com/DeveloperAromal/SynapseDB/ai/v1"
	executor "github.com/DeveloperAromal/SynapseDB/api/execute"
	"github.com/joho/godotenv"
)

func Start(ready chan<- struct{}) {
	ln, err := net.Listen("tcp", ":4538")
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

	err := godotenv.Load()
	if err != nil {
		log.Fatal("Error loading .env")
	}

	apiKey := os.Getenv("OPENROUTER_API_KEY")
	model := os.Getenv("MODEL")

	buf := make([]byte, 1024)
	n, err := conn.Read(buf)
	if err != nil {
		log.Printf("Error reading: %v", err)
		return
	}

	raw := string(buf[:n])

	query, err := generateSql.GenerateSQL(raw, apiKey, model)
	if err != nil {
		log.Printf("SQL generation failed: %v", err)
		_, _ = conn.Write([]byte("Error: " + err.Error() + "\n"))
		return
	}

	result := executor.Execute(query)

	_, err = conn.Write([]byte(result + "\n"))
	if err != nil {
		log.Printf("Error writing to client: %v", err)
	}
}
