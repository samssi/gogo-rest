package messages

import (
	"github.com/gin-gonic/gin"
	"log"
	"net/http"
)

type message struct {
	Message string `json:"message"`
}

func ReadMessage(ginContext *gin.Context) {
	message := popMessage()

	log.Printf("Returning message to the client: %v\n", message)

	if message == nil {
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

	insertMessage(message)

	ginContext.Status(http.StatusOK)
}
