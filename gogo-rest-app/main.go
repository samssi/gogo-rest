package main

import (
	"context"
	"fmt"
	"github.com/jackc/pgx/v5/pgxpool"
	"gogo-rest-app/db"
	"gogo-rest-app/routes"
	"os"
)

func createConnectionUrl() string {
	postgresHost, exists := os.LookupEnv("db_connection_string")

	if !exists {
		return "postgres://gogo:gogo@localhost:5432/gogo"
	}

	return postgresHost
}

func main() {
	connectionUrl := createConnectionUrl()
	fmt.Printf("Database connection url set as: %v\n", connectionUrl)

	dbPool, err := pgxpool.New(context.Background(), connectionUrl)
	if err != nil {
		fmt.Println("Failed to establish connection")
		os.Exit(1)
	}

	db.Pool = dbPool

	defer dbPool.Close()

	routes.Start()
}
