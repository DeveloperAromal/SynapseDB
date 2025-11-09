package main

import (
	"fmt"

	background_check "github.com/DeveloperAromal/SynapseDB/cmd/security"
	banner "github.com/DeveloperAromal/SynapseDB/cmd/security/utils"
)

func main() {
	banner.NShell_NewUserBanner()
	fmt.Print(background_check.CheckInitialRequirements("synstore/keys/master.keys.bin"))
	fmt.Print(background_check.CheckInitialRequirements("snstore/keys/master.eys.bin"))

}
