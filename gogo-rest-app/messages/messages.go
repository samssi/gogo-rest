package messages

import (
	"github.com/gin-gonic/gin"
	"log"
	"net/http"
)

type message struct {
	message string `json:"message"`
}

func ReadMessages(ginContext *gin.Context) {
	message := popMessage()

	log.Printf("Returning message to the client: %v\n", message)

	if message.message == "" {
		ginContext.Status(http.StatusNoContent)
		return
	}

	ginContext.JSON(http.StatusOK, gin.H{
		"message": message.message,
	})
}

func CreateMessage(ginContext *gin.Context) {
	var message message

	if err := ginContext.ShouldBindJSON(&message); err != nil {
		ginContext.Status(http.StatusBadRequest)
		return
	}

	log.Println("message.message")
	log.Println(message.message)

	insertMessage(message)

	ginContext.Status(http.StatusOK)
}
