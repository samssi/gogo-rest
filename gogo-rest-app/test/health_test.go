package test

import (
	"github.com/magiconair/properties/assert"
	"gogo-rest-app/routes"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHealthRoute(t *testing.T) {
	router := routes.StartApp()

	w := httptest.NewRecorder()

	req, _ := http.NewRequest("GET", "/health/", nil)

	router.ServeHTTP(w, req)

	assert.Equal(t, 200, w.Code)
}
