package generateSql

import (
	"strings"
	"testing"
)

func TestExtractSQL(t *testing.T) {
	tests := []struct {
		name    string
		content string
		want    string
	}{
		{"plain", "SELECT * FROM users;", "SELECT * FROM users;"},
		{"surrounding whitespace", "\n  SELECT * FROM users;  \n", "SELECT * FROM users;"},
		{"fence with language", "```sql\nSELECT * FROM users;\n```", "SELECT * FROM users;"},
		{"fence with CRLF", "```sql\r\nSELECT * FROM users;\r\n```", "SELECT * FROM users;"},
		{"fence without language", "```\nSELECT * FROM users;\n```", "SELECT * FROM users;"},
		{"single line fence", "```SELECT * FROM users;```", "SELECT * FROM users;"},
		{"prose around fence", "Here is the query:\n```sql\nSELECT name FROM users;\n```\nDone.", "SELECT name FROM users;"},
		{"think block", "<think>The user wants\nall rows.</think>\nSELECT * FROM users;", "SELECT * FROM users;"},
		{"think block and fence", "<think>```sql\nSELECT 1\n```</think>```sql\nSELECT * FROM users;\n```", "SELECT * FROM users;"},
		{"empty", "  ", ""},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := extractSQL(tt.content); got != tt.want {
				t.Errorf("extractSQL(%q) = %q, want %q", tt.content, got, tt.want)
			}
		})
	}
}

func TestGenerateSQLRequiresAPIKey(t *testing.T) {
	_, err := GenerateSQL("show all users", "", "")
	if err == nil || !strings.Contains(err.Error(), "OPENROUTER_API_KEY") {
		t.Fatalf("GenerateSQL with no API key: err = %v, want an error naming OPENROUTER_API_KEY", err)
	}
}
