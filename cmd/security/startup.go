package security

import (
	"encoding/json"
	"fmt"

	encryption "github.com/DeveloperAromal/SynapseDB/cmd/utils/auth/encryption"
	create_user "github.com/DeveloperAromal/SynapseDB/cmd/utils/auth/signup"
	background_process "github.com/DeveloperAromal/SynapseDB/cmd/utils/background_process"
	banner "github.com/DeveloperAromal/SynapseDB/cmd/utils/banner"
	file "github.com/DeveloperAromal/SynapseDB/cmd/utils/file"
)

type Credentials struct {
	Username string `json:"USERNAME"`
	Password string `json:"PASSWORD"`
}

func StartUp() bool {
	const filePath = "synstore/keys/master.keys.bin"
	isNewUser := background_process.CheckInitialRequirements(filePath)

	if isNewUser == false {
		banner.NShell_NewUserBanner()

		username, password := create_user.Create_account()

		hashed_password, err := encryption.HashPass(password)
		if err != nil {
			panic(err)
		}

		creds := Credentials{
			Username: username,
			Password: hashed_password,
		}

		jsonData, err := json.Marshal(creds)
		if err != nil {
			fmt.Println("Error marshalling to JSON:", err)
			panic(err)
		}

		json_output := string(jsonData)
		file.CreateBin(json_output)

		return true

	} else {

		return false
	}
}
