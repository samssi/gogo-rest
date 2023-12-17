package messages

import (
	"github.com/gin-gonic/gin"
	"net/http"
)

func FetchMessages(context *gin.Context) {
	context.JSON(http.StatusOK, gin.H{
		"message": "This should come from the DB...",
	})
}
