package test

import (
	"bytes"
	"context"
	"encoding/json"
	"github.com/gin-gonic/gin"
	"github.com/stretchr/testify/suite"
	"gogo-rest-app/db"
	"gogo-rest-app/routes"
	"net/http"
	"net/http/httptest"
	"testing"
)

type messagePayload struct {
	Message string `json:"message"`
}

func clearDatabase() {
	_, _ = db.Pool.Exec(context.Background(), "delete from message")
}

func insertTestMessageIntoDatabase(message string) {
	db.Pool.Exec(context.Background(), "insert into message (message) values ($1)", message)
}

type MessagesSuite struct {
	suite.Suite
	router *gin.Engine
}

func (s *MessagesSuite) SetupTest() {
	s.router = routes.StartApp()
	clearDatabase()
}

func (s *MessagesSuite) TestGETMessageReturnsOldestMessageFromDB() {
	insertTestMessageIntoDatabase("hey there!")
	insertTestMessageIntoDatabase("hey there2!")

	w := httptest.NewRecorder()
	req, _ := http.NewRequest("GET", "/v1/messages/", nil)

	s.router.ServeHTTP(w, req)

	s.Assert().Equal(200, w.Code)
	s.Assert().Equal("{\"message\":\"hey there!\"}", w.Body.String())
}

func (s *MessagesSuite) TestGETMessageReturnsStatusNoContentWhenDbHasNoMessages() {
	clearDatabase()
	w := httptest.NewRecorder()
	req, _ := http.NewRequest("GET", "/v1/messages/", nil)

	s.router.ServeHTTP(w, req)

	s.Assert().Equal(204, w.Code)
}

func (s *MessagesSuite) TestPostMessageIsInsertedToDB() {
	payload := messagePayload{Message: "hello!"}

	jsonPayload, _ := json.Marshal(payload)

	w := httptest.NewRecorder()
	postReq, _ := http.NewRequest("POST", "/v1/messages/", bytes.NewBuffer(jsonPayload))

	s.router.ServeHTTP(w, postReq)

	s.Assert().Equal(200, w.Code)

	getReq, _ := http.NewRequest("GET", "/v1/messages/", nil)
	s.router.ServeHTTP(w, getReq)

	s.Assert().Equal("{\"message\":\"hello!\"}", w.Body.String())
}

func TestMessagesSuite(t *testing.T) {
	suite.Run(t, new(MessagesSuite))
}
