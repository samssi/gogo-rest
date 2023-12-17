package messages

import (
	"context"
	"fmt"
	"github.com/gin-gonic/gin"
	"gogo-rest-app/db"
	"net/http"
)

func FetchMessages(ginContext *gin.Context) {
	var message string
	var err = db.Pool.QueryRow(context.Background(), "select message from message").Scan(&message)
	if err != nil {
		fmt.Printf("QueryRow failed: %v\n", err)
	}

	fmt.Printf("Returning message to the client: %v\n", message)

	ginContext.JSON(http.StatusOK, gin.H{
		"Db contains message": message,
	})
}
