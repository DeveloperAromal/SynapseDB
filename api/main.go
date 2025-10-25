package server

import (
	"fmt"
	"log"
	"net"
	"os"

	generateSql "github.com/DeveloperAromal/SynapseDB/ai/v1"
	executor "github.com/DeveloperAromal/SynapseDB/api/execute"
	"github.com/joho/godotenv"
)

func Start() {
	ln, err := net.Listen("tcp", ":4538")
	if err != nil {
		log.Fatalf("Failed to start server: %v", err)
	}
	defer ln.Close()

	fmt.Println("TCP server started on port 4538")

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

	buf := make([]byte, 1024)
	n, err := conn.Read(buf)
	if err != nil {
		log.Printf("Error reading: %v", err)
		return
	}

	raw := string(buf[:n])
	fmt.Println("Received raw:", raw)

	query, err := generateSql.GenerateSQL(raw, apiKey)
	if err != nil {
		log.Printf("SQL generation failed: %v", err)
		conn.Write([]byte("Error: failed to generate SQL\n"))
		return
	}

	result := executor.Execute(query)
	fmt.Println("Execution result:", result)

	_, err = conn.Write([]byte(result + "\n"))
	if err != nil {
		log.Printf("Error writing to client: %v", err)
	}
}
