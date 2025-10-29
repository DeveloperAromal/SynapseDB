package shell

import (
	"bufio"
	"fmt"
	"log"
	"net"
	"os"
	"strings"
	"time"
)

func Runshell() {
	host := "127.0.0.1"
	port := "4538"

	fmt.Printf("\033[38;5;208m")
	fmt.Println("======================================================")
	fmt.Println("-------------------- Synapse Shell -------------------")
	fmt.Println("======================================================")
	fmt.Printf("\033[0m\n")

	addr := net.JoinHostPort(host, port)

	var conn net.Conn
	var err error
	for i := 0; i < 10; i++ {
		conn, err = net.Dial("tcp", addr)
		if err == nil {
			break
		}
		time.Sleep(200 * time.Millisecond)
	}

	if err != nil {
		log.Fatal("Connection error:", err)
		return
	}
	defer conn.Close()

	fmt.Println("Type \033[33mexit\033[0m to close the shell.")

	reader := bufio.NewReader(os.Stdin)

	for {
		fmt.Print("\033[32m$> \033[0m")
		query, err := reader.ReadString('\n')

		if err != nil {
			log.Println("Read error:", err)
			continue
		}
		
		query = strings.TrimSpace(query)
		query = strings.ToLower(query)
		

		if strings.ToLower(query) == "exit" {
			fmt.Println("\033[31mClosing Synapse Shell...\033[0m")
			break
		}

		start := time.Now()

		conn, err := net.Dial("tcp", addr)
		if err != nil {
			log.Println("Reconnection error:", err)
			continue
		}

		_, err = conn.Write([]byte(query))
		if err != nil {
			log.Println("Send error:", err)
			conn.Close()
			continue
		}

		buffer := make([]byte, 4096)
		n, err := conn.Read(buffer)
		conn.Close()

		if err != nil {
			log.Println("Read error:", err)
			continue
		}

		end := time.Now()
		fmt.Printf("\n\033[36mLatency:\033[0m ~%dms\n", end.Sub(start).Milliseconds())
		fmt.Printf("\033[33mReceived:\033[0m %s\n\n", string(buffer[:n]))
	}

}
