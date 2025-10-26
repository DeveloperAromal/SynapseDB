package generateSql

import (
			"bytes"
			"encoding/json"
			"fmt"
			"io/ioutil"
			"net/http"
			"os"
			"strings"
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
	return fmt.Sprintf(`You are an AI that converts English instructions into SQL queries.

						Rules:
							1. Only output the SQL query, no explanations.
							2. Use proper SQL syntax for SELECT, INSERT, UPDATE, DELETE, WHERE, JOIN, GROUP BY, etc.
							3. If the instruction is ambiguous, make a reasonable assumption based on common database practices.

						Examples:
							Input: "Show all employees"
							Output: SELECT * FROM employees;

							Input: "Show the name and salary of all employees"
							Output: SELECT name, salary FROM employees;

							Now, generate SQL for this instruction:
"%s"`, rawNaturalLanguage)
}

func GenerateSQL(rawNaturalLanguage, apiKey string, model string) (string, error) {
	baseURL := "https://openrouter.ai/api/v1/chat/completions"

	if model == "" {
		model = "deepseek/deepseek-r1-0528:free"
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

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return "", err
	}

	defer resp.Body.Close()

	rawResp, _ := ioutil.ReadAll(resp.Body)

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

	return response.Choices[0].Message.Content, nil
}
