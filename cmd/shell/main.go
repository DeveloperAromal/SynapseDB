package shell

import (
			"bufio"
			"fmt"
			"log"
			"net"
			"os"
			"strings"
			"time"
			 banner "github.com/DeveloperAromal/SynapseDB/cmd/security/utils"
		)

func Runshell() {
	host := "127.0.0.1"
	port := "4538"

	banner.NShell_banner()

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

	fmt.Print("Type \033[33mexit\033[0m to close the shell.\n")

	reader := bufio.NewReader(os.Stdin)

	for {
		fmt.Print("\033[32m~> \033[0m")
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
		fmt.Printf("\033[33mReceived:\n\033[0m \n\n")

		output := string(buffer[:n])

		output = strings.ReplaceAll(output, "+", "\033[90m+\033[0m")                     
		output = strings.ReplaceAll(output, "|", "\033[90m|\033[0m")                  
		output = strings.ReplaceAll(output, "id", "\033[96mid\033[0m")                   
		output = strings.ReplaceAll(output, "name", "\033[96mname\033[0m")               
		output = strings.ReplaceAll(output, "rows in set", "\033[93mrows in set\033[0m") 

		fmt.Printf("%s\n", output)

		fmt.Printf("\n\033[36mLatency:\033[0m ~ %dms\n", end.Sub(start).Milliseconds())

	}

}
