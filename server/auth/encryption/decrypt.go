package encryption

import (
	"golang.org/x/crypto/bcrypt"
)

func ValidateHash(plain, hashed string) bool {
	err := bcrypt.CompareHashAndPassword([]byte(hashed), []byte(plain))
	return err == nil
}
