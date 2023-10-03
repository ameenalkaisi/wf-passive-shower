package main

import (
	"bytes"
	"io"
	"log"
	"mime/multipart"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

// Test uploading image to server
func TestUploadScreenshot(t *testing.T) {
	// Create a buffer to store the request body
	var buf bytes.Buffer

	// Create a new multipart writer with the buffer
	multipartWriter := multipart.NewWriter(&buf)

	// Add a file to the request
	file, err := os.Open("test/first.png")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	// Create a new form field
	fw, err := multipartWriter.CreateFormFile("screenshot", "file.png")
	if err != nil {
		log.Fatal(err)
	}

	// Copy the contents of the file to the form field
	if _, err := io.Copy(fw, file); err != nil {
		log.Fatal(err)
	}

	// Close the multipart writer to finalize the request
	multipartWriter.Close()

	req := httptest.NewRequest("POST", "http://localhost:8080/analyze-wf-ss", &buf)
	req.Header.Set("Content-Type", multipartWriter.FormDataContentType())

	// Send the request
	router := SetupRouter()

	recorder := httptest.NewRecorder()
	router.ServeHTTP(recorder, req)

	assert.Equal(t, http.StatusOK, recorder.Code)
}
