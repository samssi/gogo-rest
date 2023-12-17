package messages

import (
	"context"
	"github.com/gin-gonic/gin"
	"gogo-rest-app/db"
	"log"
	"net/http"
)

type message struct {
	message string
}

func queryMessages() string {
	var message string
	err := db.Pool.QueryRow(context.Background(), "select message from message").Scan(&message)
	if err != nil {
		log.Printf("QueryRow failed: %v\n", err)
	}

	return message
}

func insertMessage(message message) {
	log.Printf("Adding message to the db: %v\n", message.message)

	_, err := db.Pool.Exec(context.Background(), "insert into message (message) values ($1)", message.message)
	if err != nil {
		log.Printf("Insert exec failed: %v\n", err)
	}
}

func ReadMessages(ginContext *gin.Context) {
	message := queryMessages()

	log.Printf("Returning message to the client: %v\n", message)

	ginContext.JSON(http.StatusOK, gin.H{
		"message": message,
	})
}

func CreateMessage(ginContext *gin.Context) {
	var message message

	if err := ginContext.BindJSON(&message); err != nil {
		ginContext.Status(http.StatusBadRequest)
		return
	}

	log.Println("message.message")
	log.Println(message.message)

	insertMessage(message)

	ginContext.Status(http.StatusOK)
}

func UpdateMessage(ginContext *gin.Context) {

}

func DeleteMessage(ginContext *gin.Context) {

}
