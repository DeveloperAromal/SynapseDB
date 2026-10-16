package generateSql

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"os"
	"regexp"
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

// Free models can take a while to answer, but a stalled request must not hang
// the client connection forever.
var httpClient = &http.Client{Timeout: 2 * time.Minute}

var (
	thinkBlock = regexp.MustCompile(`(?s)<think>.*?</think>`)
	codeFence  = regexp.MustCompile("(?s)```(?:[A-Za-z]+\\r?\\n)?(.*?)```")
)

func promptTemplate(rawNaturalLanguage string) string {
	return fmt.Sprintf(`You are an AI that converts English instructions into SQL for SynapseDB.

Rules:
1. Output exactly one SQL statement and nothing else: no explanations and no Markdown code fences.
2. Only use the statements SynapseDB supports:
   CREATE TABLE <table> (<column> <type>, ...) where <type> is INTEGER or TEXT
   INSERT INTO <table> (<column>, ...) VALUES (<value>, ...) and always include the column list
   SELECT * or SELECT <column>, ... FROM <table>, optionally followed by WHERE <column> <operator> <value>
   with a single comparison using =, !=, <, >, <= or >=
3. Put text values in single quotes.
4. If the instruction is ambiguous, make a reasonable assumption based on common database practices.

Examples:
Input: "Show all employees"
Output: SELECT * FROM employees;

Input: "Show the name and salary of all employees"
Output: SELECT name, salary FROM employees;

Input: "Add an employee named Asha who is 29"
Output: INSERT INTO employees (name, age) VALUES ('Asha', 29);

Now, generate SQL for this instruction:
"%s"`, rawNaturalLanguage)
}

func GenerateSQL(rawNaturalLanguage, apiKey string, model string) (string, error) {
	baseURL := "https://openrouter.ai/api/v1/chat/completions"

	if apiKey == "" {
		return "", errors.New("OPENROUTER_API_KEY is not set; write the query in SQL or set the key in .env")
	}

	if model == "" {
		model = "deepseek/deepseek-r1-0528:free"
	}

	requestBody := ChatRequest{
		Model: model,
		Messages: []Message{
			{Role: "user", Content: promptTemplate(rawNaturalLanguage)},
		},
	}

	bodyBytes, err := json.Marshal(requestBody)
	if err != nil {
		return "", err
	}

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

	resp, err := httpClient.Do(req)
	if err != nil {
		return "", err
	}

	defer resp.Body.Close()

	rawResp, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("failed to read response: %w", err)
	}

	// surface non-2xx responses and non-JSON bodies (helps with OpenRouter privacy pages)
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return "", fmt.Errorf("non-2xx response: %d | body: %s", resp.StatusCode, string(rawResp))
	}

	contentType := resp.Header.Get("Content-Type")
	if contentType == "" || !strings.Contains(strings.ToLower(contentType), "application/json") {
		return "", fmt.Errorf("unexpected content-type: %s | body: %s", contentType, string(rawResp))
	}

	var response ChatResponse
	if err := json.Unmarshal(rawResp, &response); err != nil {
		return "", fmt.Errorf("failed to decode: %v | raw: %s", err, string(rawResp))
	}

	if len(response.Choices) == 0 {
		return "", fmt.Errorf("no choices returned from model | raw: %s", string(rawResp))
	}

	sql := extractSQL(response.Choices[0].Message.Content)
	if sql == "" {
		return "", fmt.Errorf("model returned no SQL | raw: %s", string(rawResp))
	}

	return sql, nil
}

// extractSQL removes what models often wrap around the statement despite the
// prompt: reasoning blocks and Markdown code fences.
func extractSQL(content string) string {
	content = thinkBlock.ReplaceAllString(content, "")

	if match := codeFence.FindStringSubmatch(content); match != nil {
		content = match[1]
	}

	return strings.TrimSpace(content)
}
