package shell

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"log"
	"net"
	"os"
	"strings"
	"time"

	startup "github.com/DeveloperAromal/SynapseDB/cmd/security"
	validate "github.com/DeveloperAromal/SynapseDB/cmd/utils/auth/login"
	banner "github.com/DeveloperAromal/SynapseDB/cmd/utils/banner"
)

func Runshell() {

	isNewUser := startup.StartUp()

	if !isNewUser {
		banner.NShell_banner()

		isValid := validate.ValidateUser()
		if !isValid {
			fmt.Println("Invalid username or password.")
			return
		}

		time.Sleep(700 * time.Millisecond)
	}

	host := "127.0.0.1"
	port := "4538"

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
	// This connection only confirms the server is up; every query opens its
	// own. Holding it open would keep a server goroutine blocked on it.
	conn.Close()

	fmt.Print("Type \033[33mexit\033[0m to close the shell.\n")

	reader := bufio.NewReader(os.Stdin)

	for {
		fmt.Print("\033[32m$ \033[0m")
		line, err := reader.ReadString('\n')
		if err != nil && !errors.Is(err, io.EOF) {
			log.Println("Read error:", err)
			break
		}
		// io.EOF means stdin was closed (Ctrl+D, Ctrl+Z or the end of piped
		// input): run what was typed before it, then exit.
		atEOF := err != nil

		query := strings.TrimSpace(line)

		if strings.EqualFold(query, "exit") {
			break
		}

		// An empty write never reaches the server, so sending one would leave
		// the shell waiting for a reply forever.
		if query != "" {
			sendQuery(addr, query)
		}

		if atEOF {
			fmt.Println()
			break
		}
	}

	fmt.Println("\033[31mClosing Synapse Shell...\033[0m")
}

func sendQuery(addr, query string) {
	start := time.Now()

	conn, err := net.Dial("tcp", addr)
	if err != nil {
		log.Println("Reconnection error:", err)
		return
	}
	defer conn.Close()

	_, err = conn.Write([]byte(query))
	if err != nil {
		log.Println("Send error:", err)
		return
	}

	// The server closes the connection after it replies, so read until EOF
	// instead of a single fixed-size read that cuts long results short.
	response, err := io.ReadAll(conn)
	if err != nil {
		log.Println("Read error:", err)
		return
	}

	end := time.Now()
	fmt.Printf("\033[33mReceived:\n\033[0m \n\n")

	output := string(response)

	output = strings.ReplaceAll(output, "+", "\033[90m+\033[0m")
	output = strings.ReplaceAll(output, "|", "\033[90m|\033[0m")
	output = strings.ReplaceAll(output, "id", "\033[96mid\033[0m")
	output = strings.ReplaceAll(output, "name", "\033[96mname\033[0m")
	output = strings.ReplaceAll(output, "rows in set", "\033[93mrows in set\033[0m")

	fmt.Printf("%s\n", output)

	fmt.Printf("\n\033[36mLatency:\033[0m ~ %dms\n", end.Sub(start).Milliseconds())
}
