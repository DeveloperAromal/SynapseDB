package background_process

import (
			"os"
			// "strconv"
			binConverter "github.com/DeveloperAromal/SynapseDB/cmd/utils/file"
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


		// dec_bin_str := strconv.FormatUint(uint64(dec_bin), 10)

		if dec_bin == "" {
			// File exists but is empty or corrupted, delete it
			os.Remove(path)
			return false

		} else{
			return true

		}
	 }

	 return false

}
