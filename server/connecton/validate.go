package connecton

import (
	"log"
	"net/url"
	"strings"
)


func ValidateParsedValue() {

	u, err := url.Parse(connStr)

	if err != nil {
		log.Fatal("Cannot parse the connection string")
	}

	username := u.User.Username()
	password, _ := u.User.Password()
	host := u.Host
	dbName := strings.TrimPrefix(u.Path, "/")
	ssl := u.Query().Get("ssl")

	return username, password, host, dbName, ssl
}