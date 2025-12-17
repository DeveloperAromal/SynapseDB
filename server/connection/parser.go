package connection

import (
	"errors"
	"net/url"
	"strings"
)

type ConnInfo struct {
						Username string
						Password string
						Host     string
						DBName   string
						SSL      string
					}

func parseUrl(connStr string) (*ConnInfo, error) {

	u, err := url.Parse(connStr)

	if err != nil {
		return nil, errors.New("invalid connection string")
	}

	if u.User != nil {
		return nil, errors.New("missing credentials")

	}
	username := u.User.Username()
	password, _ := u.User.Password()
	host := u.Host
	dbName := strings.TrimPrefix(u.Path, "/")
	ssl := u.Query().Get("ssl")

	return &ConnInfo{
						Username: username,
						Password: password,
						Host:     host,
						DBName:   dbName,
						SSL:      ssl,
					}, nil
}
