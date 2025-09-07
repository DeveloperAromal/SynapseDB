package generateSql

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
)

type ChatRequest struct {
	Model   string `json:"model"`
	Messages []Message `json:"messages"`
}

type Message struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

func promptTemplate(rawNaturalLanguage string) string {
	prompt := fmt.Sprintf(`You are an AI that converts English instructions into SQL queries.

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
							"%s"
							`, rawNaturalLanguage)
								return prompt
							}

func GenerateSQL(rawNaturalLanguage, apiKey string) (string, error) {
	baseURL := "https://openrouter.ai/api/v1/chat/completions"

	requestBody := ChatRequest{
		Model: "gpt-4.1-mini",
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

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	respBody, _ := ioutil.ReadAll(resp.Body)
	return string(respBody), nil
}


