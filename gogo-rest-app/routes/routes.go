package routes

import (
	"fmt"
	"github.com/gin-gonic/gin"
	"gogo-rest-app/health"
	"gogo-rest-app/messages"
)

type routes struct {
	router *gin.Engine
}

func (routes routes) messageRouterGroup(routerGroup *gin.RouterGroup) {
	messageRouterGroup := routerGroup.Group("/messages")

	messageRouterGroup.GET("/", messages.FetchMessages)

}

func (routes routes) healthCheckRouterGroup(routerGroup *gin.RouterGroup) {
	healthCheckRouterGroup := routerGroup.Group("/health")

	healthCheckRouterGroup.GET("/", health.ReturnStatus)
}

func Start() {
	routes := routes{
		gin.Default(),
	}

	root := routes.router.Group("/")
	v1 := routes.router.Group("/v1")

	routes.healthCheckRouterGroup(root)
	routes.messageRouterGroup(v1)

	err := routes.router.Run()
	if err != nil {
		fmt.Println("Failed to run gin server")
	}
}
