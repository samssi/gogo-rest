package main

import (
	"fmt"
	"gogo-rest-app/db"
	"gogo-rest-app/routes"
)

func main() {
	router := routes.StartApp()

	defer db.Pool.Close()

	if err := router.Run(":8080"); err != nil {
		fmt.Println("Failed to run gin server")
	}
}
