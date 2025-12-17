package connection

import (
	auth "github.com/DeveloperAromal/SynapseDB/server/auth"
)

type ConnStr struct {
	Username string
	Password string
	Host     string
	DBName   string
	SSL      string
}

func ValidateParsedValue(conn *ConnStr) bool {
	validatePassword, validateUsername := auth.ValidateUser(
		conn.Password,
		conn.Username,
	)

	if validatePassword && validateUsername {
		return true
	}

	return false
}
