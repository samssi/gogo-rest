package rest

import (
	"fmt"
	"github.com/gin-gonic/gin"
	"net/http"
)

func Start() {
	r := gin.Default()
	r.GET("/ping", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	})

	err := r.Run()
	if err != nil {
		fmt.Println("Failed to run gin server")
	}
}
