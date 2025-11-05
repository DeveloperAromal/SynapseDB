package encryption

import (
    		"hash/fnv"
		)



func hashPass(text string) uint32 {
    hash := fnv.New32()
    hash.Write([]byte(text))

    return hash.Sum32()
}


func validateHash(text string, hashedPass uint32) bool{
	hash := fnv.New32()
	hash.Write([]byte(text))
	
	if hash.Sum32() == hashedPass {
		return true
	} else {
		return false
	}
}



