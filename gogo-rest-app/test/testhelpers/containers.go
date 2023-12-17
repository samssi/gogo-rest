package testhelpers

import (
	"context"
	"github.com/testcontainers/testcontainers-go"
	"github.com/testcontainers/testcontainers-go/modules/postgres"
	"github.com/testcontainers/testcontainers-go/wait"
	"time"
)

type PostgresContainer struct {
	*postgres.PostgresContainer
	ConnectionString string
}

func CreatePostgresContainer(ctx context.Context) (*PostgresContainer, error) {
	pgContainer, err := postgres.RunContainer(ctx, testcontainers.WithImage("gogo-postgres:dev"),
		postgres.WithDatabase("gogo"),
		postgres.WithUsername("gogo"),
		postgres.WithPassword("gogo"),
		testcontainers.WithWaitStrategy(
			wait.ForLog("database system is ready to accept connections").WithOccurrence(2).WithStartupTimeout(5*time.Second)),
	)

	if err != nil {
		return nil, err
	}

	connectionString, err := pgContainer.ConnectionString(ctx, "sslmode=disable")

	if err != nil {
		return nil, err
	}

	return &PostgresContainer{
		PostgresContainer: pgContainer,
		ConnectionString:  connectionString,
	}, nil
}
