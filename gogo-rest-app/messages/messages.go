package messages

import (
	"github.com/gin-gonic/gin"
	"log"
	"net/http"
)

type message struct {
	Message string `json:"message" binding:"required"`
}

func ReadMessage(ginContext *gin.Context) {
	dbMessage := popMessage()

	if dbMessage == nil {
		ginContext.Status(http.StatusNoContent)
		return
	}

	log.Printf("Returning message to the client: %v\n", dbMessage.message)

	message := message{
		Message: dbMessage.message,
	}

	ginContext.JSON(http.StatusOK, message)
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
