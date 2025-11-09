package security

import (
			"os"
			"strconv"
			binConverter "github.com/DeveloperAromal/SynapseDB/cmd/security/utils"
	   )

func IfPathExists(path string) bool{ 
	if _, err := os.Stat(path); os.IsNotExist(err) {
		return false

	} else {
		return true

	}
}



func CheckInitialRequirements(path string) bool { 

	 if IfPathExists(path) {
		dec_bin := binConverter.ReadBin(path)


		dec_bin_str := strconv.FormatUint(uint64(dec_bin), 10)

		if len(dec_bin_str) < 6 {
			return false

		} else{
			return true

		}
	 }

	 return false

}
