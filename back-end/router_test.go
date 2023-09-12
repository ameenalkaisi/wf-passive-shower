package main

import (
	"mime/multipart"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
)

// TestHelloEmpty calls greetings.Hello with an empty string,
// checking for an error.
func TestHelloEmpty(t *testing.T) {
	r := SetupRouter()
	// r.GET("/companies", GetCompaniesHandler)
	req, _ := http.NewRequest("GET", "/companies", nil)

	// do something here
	multipart.NewWriter()

	// something with httptest

	r.ServeHTTP(w, req)

	// var companies []Company
	// json.Unmarshal(w.Body.Bytes(), &companies)

	assert.Equal(t, http.StatusOK, w.Code)
	// assert.NotEmpty(t, companies)
}
