package routes

import (
	"github.com/gin-gonic/gin"
	"gogo-rest-app/health"
	"gogo-rest-app/messages"
)

type routes struct {
	router *gin.Engine
}

func (routes routes) messageRouterGroup(routerGroup *gin.RouterGroup) {
	messageRouterGroup := routerGroup.Group("/messages")
	messageRouterGroup.Use(gin.Logger())

	messageRouterGroup.GET("/", messages.ReadMessage)
	messageRouterGroup.POST("/", messages.CreateMessage)

}

func (routes routes) healthCheckRouterGroup(routerGroup *gin.RouterGroup) {
	healthCheckRouterGroup := routerGroup.Group("/health")

	healthCheckRouterGroup.GET("/", health.ReturnStatus)
}

func setupRouter() *gin.Engine {
	routes := routes{
		gin.Default(),
	}

	root := routes.router.Group("/")
	v1 := routes.router.Group("/v1")

	routes.healthCheckRouterGroup(root)
	routes.messageRouterGroup(v1)

	return routes.router
}
