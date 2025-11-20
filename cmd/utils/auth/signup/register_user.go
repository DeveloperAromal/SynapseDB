package signup

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"golang.org/x/term"
)

func Create_account() (string, string) {
	reader := bufio.NewReader(os.Stdin)

	fmt.Print("Enter a username: ")
	username, _ := reader.ReadString('\n')
	username = strings.TrimSpace(username)

	fmt.Print("Enter a password: ")
	bytePassword, _ := term.ReadPassword(int(os.Stdin.Fd()))
	fmt.Print()
	password := strings.TrimSpace(string(bytePassword))

	fmt.Println("")

	
	fmt.Print("Confirm password: ")
	byteCPassword, _ := term.ReadPassword(int(os.Stdin.Fd()))
	fmt.Print()
	c_password := strings.TrimSpace(string(byteCPassword))

	if password != c_password {
		fmt.Println("Password does not match the previous entered")
		os.Exit(1)
	}

	return username, password
}

