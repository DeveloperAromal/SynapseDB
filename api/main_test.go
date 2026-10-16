package api

import (
	"strings"
	"testing"
)

func TestLooksLikeSQL(t *testing.T) {
	tests := []struct {
		input string
		want  bool
	}{
		{"SELECT * FROM users;", true},
		{"select name from users", true},
		{"  Insert INTO users (id) VALUES (1)", true},
		{"CREATE TABLE users (id INTEGER)", true},
		{"show all users", false},
		{"selected users", false},
		{"", false},
		{"   ", false},
	}

	for _, tt := range tests {
		if got := looksLikeSQL(tt.input); got != tt.want {
			t.Errorf("looksLikeSQL(%q) = %v, want %v", tt.input, got, tt.want)
		}
	}
}

func TestProcessQueryRejectsEmptyInput(t *testing.T) {
	if got := processQuery(""); got != "Error: empty query" {
		t.Errorf("processQuery(\"\") = %q, want %q", got, "Error: empty query")
	}
}

func TestProcessQueryWithoutAPIKeyExplainsWhy(t *testing.T) {
	t.Setenv("OPENROUTER_API_KEY", "")

	got := processQuery("show all users")
	if !strings.Contains(got, "OPENROUTER_API_KEY") {
		t.Errorf("processQuery for English input without an API key = %q, want an error naming OPENROUTER_API_KEY", got)
	}
}
