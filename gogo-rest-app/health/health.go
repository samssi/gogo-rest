package health

import "github.com/gin-gonic/gin"

func ReturnStatus(context *gin.Context) {
	context.Status(200)
}
