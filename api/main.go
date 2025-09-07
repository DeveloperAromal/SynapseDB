package main

import (
	"fmt"
	"net"

	executor "github.com/DeveloperAromal/SynapseDB/api/execute"
)

func main() {

	ln, err := net.Listen("tcp", ":4538")

	if err != nil {
		fmt.Println(err)
	}

	fmt.Println("TCP server started on port 4538")

	for {
		conn, err := ln.Accept()

		if err != nil {
			fmt.Println(err)
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
        fmt.Println("Error reading:", err)
        return
    }

    query := string(buf[:n])
    fmt.Println("Received query:", query)

    result := executor.Execute(query)
    fmt.Println("Rust result:", result)

    conn.Write([]byte(result + "\n"))
}

