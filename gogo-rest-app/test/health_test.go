package test

import (
	"github.com/stretchr/testify/suite"
	"gogo-rest-app/routes"
	"net/http"
	"net/http/httptest"
	"testing"
)

type HealthSuite struct {
	suite.Suite
}

func (s *HealthSuite) TestHealthRoute() {
	router := routes.StartApp()

	w := httptest.NewRecorder()

	req, _ := http.NewRequest("GET", "/health/", nil)

	router.ServeHTTP(w, req)

	s.Assert().Equal(200, w.Code)
}

func TestHealthSuite(t *testing.T) {
	suite.Run(t, new(HealthSuite))
}
