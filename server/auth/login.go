package login

import (
	"encoding/json"
	"errors"
	"fmt"
	"strings"

	file "github.com/DeveloperAromal/SynapseDB/cmd/utils/file"
	encryption "github.com/DeveloperAromal/SynapseDB/server/auth/encryption"
)

type Credentials struct {
	Username string `json:"USERNAME"`
	Password string `json:"PASSWORD"`
}

var cachedCredentials *Credentials

func readAndParseCredentials(filePath string) (*Credentials, error) {
	if cachedCredentials != nil {
		return cachedCredentials, nil
	}

	contents := file.ReadBin(filePath)

	contents = strings.TrimSpace(contents)
	contents = strings.Trim(contents, "\x00")

	var creds Credentials

	err := json.Unmarshal([]byte(contents), &creds)
	if err != nil {
		return nil, fmt.Errorf("failed to parse credentials file: %w", err)
	}

	cachedCredentials = &creds

	return &creds, nil
}
func GetPassword(keyContents string) (string, error) {
	const filePath = "synstore/keys/master.keys.bin"

	creds, err := readAndParseCredentials(filePath)
	if err != nil {
		return "", errors.New("PASSWORD not found")
	}
	return creds.Password, nil
}

func GetUsername(keyContents string) (string, error) {
	const filePath = "synstore/keys/master.keys.bin"

	creds, err := readAndParseCredentials(filePath)
	if err != nil {
		return "", errors.New("USERNAME not found")
	}
	return creds.Username, nil
}
func ValidateUser(password string, username string) (bool, bool) {

	bin_hash_pass, err := GetPassword("")
	if err != nil {
		panic(err)
	}

	stored_username, err := GetUsername("")
	if err != nil {
		panic(err)
	}

	isValidUsername := username == stored_username
	isValidPassword := encryption.ValidateHash(password, bin_hash_pass)

	return isValidPassword, isValidUsername
}
