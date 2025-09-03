package main

import (
	"fmt"
	"net"
)

func main() {

	ln, err := net.Listen("tcp", ":4538")

	if err != nil {
		fmt.Println(err)
	}

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
	_, err := conn.Read(buf)

	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Printf("Received: %s", buf)
}
