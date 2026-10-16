package connection

import "testing"

func TestParseUrl(t *testing.T) {
	info, err := parseUrl("synapse://admin:secret@localhost:4538/shop?ssl=true")
	if err != nil {
		t.Fatalf("parseUrl returned error: %v", err)
	}

	want := ConnInfo{
		Username: "admin",
		Password: "secret",
		Host:     "localhost:4538",
		DBName:   "shop",
		SSL:      "true",
	}
	if *info != want {
		t.Errorf("parseUrl = %+v, want %+v", *info, want)
	}
}

func TestParseUrlRequiresCredentials(t *testing.T) {
	for _, connStr := range []string{
		"synapse://localhost:4538/shop",
		"synapse://:secret@localhost:4538/shop",
	} {
		if _, err := parseUrl(connStr); err == nil {
			t.Errorf("parseUrl(%q) succeeded, want a missing credentials error", connStr)
		}
	}
}
