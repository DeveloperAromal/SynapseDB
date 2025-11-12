package login

import (
	"encoding/base64"
	"errors"
	"fmt"
	"os"
	"strings"

	encryption "github.com/DeveloperAromal/SynapseDB/cmd/utils/auth/encryption"
	file "github.com/DeveloperAromal/SynapseDB/cmd/utils/file"
	"golang.org/x/term"
)

func GetPassword(keyContents string) (string, error) {
	lines := strings.Split(keyContents, "\n")
	for _, line := range lines {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}

		if strings.HasPrefix(line, "PASSWORD=") {
			parts := strings.SplitN(line, "=", 2)
			if len(parts) == 2 {
				return strings.TrimSpace(parts[1]), nil
			}
		}
	}

	return "", errors.New("PASSWORD not found")
}


func ValidateUser() bool {
	fmt.Print("Enter your password: ")
	bytePassword, _ := term.ReadPassword(int(os.Stdin.Fd()))
	fmt.Print()
	password := strings.TrimSpace(string(bytePassword))
	fmt.Println("")

	const filePath = "synstore/keys/master.keys.bin"

	contents, _ := base64.StdEncoding.DecodeString(file.ReadBin(filePath))

	bin_hash_pass, err := GetPassword(string(contents))

	if err != nil {
		panic(err)
	}


	isValidUser := encryption.ValidateHash(password, bin_hash_pass)

	return isValidUser
}
