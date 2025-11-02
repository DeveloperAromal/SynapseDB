package generateSql

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"strings"
	"time"
)

type ChatRequest struct {
	Model    string    `json:"model"`
	Messages []Message `json:"messages"`
}

type Message struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

type ChatResponse struct {
	Choices []struct {
		Message struct {
			Role    string `json:"role"`
			Content string `json:"content"`
		} `json:"message"`
	} `json:"choices"`
}

func promptTemplate(rawNaturalLanguage string) string {
	return fmt.Sprintf(`You are an AI SQL generator. 
						Your job: Convert natural English instructions into **only SQL queries**.

						### RULES
							1. Output ONLY a valid SQL query — no explanations, no extra words, no apologies.
							2. Use proper SQL syntax (SELECT, INSERT, UPDATE, DELETE, WHERE, JOIN, etc.).
							3. If the instruction is ambiguous, make a reasonable assumption.
							4. NEVER include markdown formatting, explanations, or natural language text.

						### Examples
							Instruction: "Show all employees"
							SQL: SELECT * FROM employees;

							Instruction: "Show the name and salary of all employees"
							SQL: SELECT name, salary FROM employees;

							Instruction: "Add a user named John"
							SQL: INSERT INTO users (name) VALUES ('John');

							### Instruction:
							"%s"

							Now output ONLY the SQL query:`, rawNaturalLanguage)
}

var client = &http.Client{
	Timeout: 15 * time.Second,
}

func GenerateSQL(rawNaturalLanguage, apiKey string, model string) (string, error) {
	baseURL := "https://openrouter.ai/api/v1/chat/completions"

	if model == "" {
		model = "meta-llama/llama-3.1-8b-instruct:free"
	}

	requestBody := ChatRequest{
		Model: model,
		Messages: []Message{
			{Role: "user", Content: promptTemplate(rawNaturalLanguage)},
		},
	}

	bodyBytes, _ := json.Marshal(requestBody)

	req, err := http.NewRequest("POST", baseURL, bytes.NewBuffer(bodyBytes))
	if err != nil {
		return "", err
	}

	req.Header.Set("Authorization", "Bearer "+apiKey)
	req.Header.Set("Content-Type", "application/json")

	if ref := os.Getenv("OPENROUTER_REFERER"); ref != "" {
		req.Header.Set("HTTP-Referer", ref)
	}
	if title := os.Getenv("OPENROUTER_TITLE"); title != "" {
		req.Header.Set("X-Title", title)
	}

	resp, err := client.Do(req)
	if err != nil {
		return "", fmt.Errorf("request failed: %v", err)
	}
	defer resp.Body.Close()

	rawResp, _ := ioutil.ReadAll(resp.Body)

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return "", fmt.Errorf("non-2xx response: %d | body: %s", resp.StatusCode, string(rawResp))
	}

	contentType := resp.Header.Get("Content-Type")
	if contentType == "" || !strings.Contains(strings.ToLower(contentType), "application/json") {
		return "", fmt.Errorf("unexpected content-type: %s | body: %s", contentType, string(rawResp))
	}

	var response ChatResponse
	if err := json.Unmarshal(rawResp, &response); err != nil {
		return "", fmt.Errorf("failed to decode response: %v | raw: %s", err, string(rawResp))
	}

	if len(response.Choices) == 0 {
		return "", fmt.Errorf("no choices returned from model | raw: %s", string(rawResp))
	}

	sqlOutput := cleanSQL(response.Choices[0].Message.Content)

	if sqlOutput == "" {
		return "", fmt.Errorf("empty SQL output | raw: %s", string(rawResp))
	}

	return sqlOutput, nil
}

func cleanSQL(output string) string {
	sql := strings.TrimSpace(output)
	sql = strings.ReplaceAll(sql, "```sql", "")
	sql = strings.ReplaceAll(sql, "```", "")
	sql = strings.ReplaceAll(sql, "`", "")
	sql = strings.TrimSpace(sql)

	if strings.Contains(strings.ToLower(sql), "sorry") ||
		strings.Contains(strings.ToLower(sql), "apologize") ||
		strings.Contains(strings.ToLower(sql), "please") ||
		strings.Contains(strings.ToLower(sql), "instruction") ||
		strings.Contains(strings.ToLower(sql), "clarify") {
		return ""
	}

	validPrefixes := []string{"select", "insert", "update", "delete", "create", "drop"}
	sqlLower := strings.ToLower(sql)
	for _, prefix := range validPrefixes {
		if strings.HasPrefix(sqlLower, prefix) {
			return sql
		}
	}

	return ""
}
