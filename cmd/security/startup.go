package security

import (
	"encoding/base64"
	"fmt"

	encryption "github.com/DeveloperAromal/SynapseDB/cmd/utils/auth/encryption"
	create_user "github.com/DeveloperAromal/SynapseDB/cmd/utils/auth/signup"
	background_process "github.com/DeveloperAromal/SynapseDB/cmd/utils/background_process"
	banner "github.com/DeveloperAromal/SynapseDB/cmd/utils/banner"
	file "github.com/DeveloperAromal/SynapseDB/cmd/utils/file"
)

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

		key_contents := fmt.Sprintf(`
				USERNAME=%s
				PASSWORD=%s`, 
				username, hashed_password)

		encode := base64.StdEncoding.EncodeToString([]byte(key_contents))
		
		file.CreateBin(encode)

		return true

	} else {

		return false
	}
}
