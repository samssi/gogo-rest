package main

import (
	"context"
	"fmt"
	"github.com/jackc/pgx/v5/pgxpool"
	"gogo-rest-app/db"
	"gogo-rest-app/routes"
	"os"
)

func main() {
	databaseUrl := "postgres://gogo:gogo@localhost:5432"

	dbPool, err := pgxpool.New(context.Background(), databaseUrl)
	if err != nil {
		fmt.Println("Failed to establish connection")
		os.Exit(1)
	}

	db.Pool = dbPool

	defer dbPool.Close()

	routes.Start()
}
