package security

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"golang.org/x/term"
	background_process "github.com/DeveloperAromal/SynapseDB/cmd/utils/background_process"
	banner "github.com/DeveloperAromal/SynapseDB/cmd/utils/banner"
)
	   
func StartupCheck() (username string, password string,ok  bool){
	isNewUser := background_process.CheckInitialRequirements("")

	if isNewUser == false {
		banner.NShell_NewUserBanner()

		reader := bufio.NewReader(os.Stdin)

		fmt.Println("Enter a username: ")
		username, _ := reader.ReadString('\n')
		username = strings.TrimSpace(username)

		fmt.Println("Enter a password: ")
		bytePassword, _ := term.ReadPassword(int(os.Stdin.Fd()))
		fmt.Println()
		password, _ := reader.ReadString('\n')
		password = strings.TrimSpace(string(bytePassword))

		fmt.Println("Confirm password: ")
		byteCPassword, _ := term.ReadPassword(int(os.Stdin.Fd()))
		fmt.Println()
		c_password, _ := reader.ReadString('\n')
		c_password = strings.TrimSpace(string(byteCPassword))


		if c_password == password {
			return username, password, true

		} else {
			fmt.Println("Wrong password")
			return "", "", false
		}
	}
	return "", "", false
}