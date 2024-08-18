// Package main implements a simple fake mail server for testing purposes.
//
// This server accepts email send requests via HTTP POST to "/v3/mail/send" and
// stores them in memory. It also provides a status page at "/status" to view
// all stored emails.
//
// Usage:
//
//	go run main.go <port>
//
// Where <port> is the port number on which the server should listen.
package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"sync"
)

// SendEmailRequest represents the structure of an email send request.
type SendEmailRequest struct {
	Personalizations []Personalization `json:"personalizations"`
	From             From              `json:"from"`
	Subject          string            `json:"subject"`
	Content          []Content         `json:"content"`
}

// Personalization contains the recipient information for an email.
type Personalization struct {
	To []To `json:"to"`
}

// To represents a single recipient of an email.
type To struct {
	Email string `json:"email"`
	Name  string `json:"name"`
}

// From represents the sender of an email.
type From struct {
	Email string `json:"email"`
}

// Content represents the content of an email.
type Content struct {
	Type  string `json:"type"`
	Value string `json:"value"`
}

var (
	emails []SendEmailRequest
	mu     sync.Mutex
)

func main() {
	if len(os.Args) < 2 {
		log.Fatal("Please provide a port number as an argument")
	}

	port := os.Args[1]

	http.HandleFunc("/v3/mail/send", handleSendMail)
	http.HandleFunc("/", handleStatus)

	log.Printf("Starting server on port %s\n", port)
	log.Fatal(http.ListenAndServe(":"+port, nil))
}

// handleSendMail processes POST requests to /v3/mail/send.
// It decodes the JSON payload into a SendEmailRequest struct and stores it in memory.
func handleSendMail(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req SendEmailRequest
	err := json.NewDecoder(r.Body).Decode(&req)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	mu.Lock()
	emails = append(emails, req)
	mu.Unlock()

	w.WriteHeader(http.StatusOK)
	fmt.Fprintf(w, "Email received and stored")
}

// handleStatus processes GET requests to /.
// It generates an HTML page displaying all stored emails.
func handleStatus(w http.ResponseWriter, r *http.Request) {
	mu.Lock()
	defer mu.Unlock()

	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<h1>Stored Emails</h1>")
	for i, email := range emails {
		fmt.Fprintf(w, "<h2>Email %d</h2>", i+1)
		fmt.Fprintf(w, "<p><strong>From:</strong> %s</p>", email.From.Email)
		fmt.Fprintf(w, "<p><strong>Subject:</strong> %s</p>", email.Subject)
		fmt.Fprintf(w, "<p><strong>To:</strong></p><ul>")
		for _, p := range email.Personalizations {
			for _, to := range p.To {
				fmt.Fprintf(w, "<li>%s (%s)</li>", to.Name, to.Email)
			}
		}
		fmt.Fprintf(w, "</ul>")
		fmt.Fprintf(w, "<p><strong>Content:</strong></p><ul>")
		for _, c := range email.Content {
			fmt.Fprintf(w, "<li>Type: %s<br>Value: %s</li>", c.Type, c.Value)
		}
		fmt.Fprintf(w, "</ul><hr>")
	}
}
