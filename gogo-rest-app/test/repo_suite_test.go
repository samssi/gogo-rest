package test

import (
	"context"
	"github.com/stretchr/testify/suite"
	"gogo-rest-app/test/testhelpers"
	"log"
	"testing"
)

type RepoSuite struct {
	suite.Suite
	pgContainer *testhelpers.PostgresContainer
	ctx         context.Context
}

func (suite *RepoSuite) SetupSuite() {
	suite.ctx = context.Background()
	pgContainer, err := testhelpers.CreatePostgresContainer(suite.ctx)
	if err != nil {
		log.Fatal(err)
	}

	suite.pgContainer = pgContainer

}

func (suite *RepoSuite) TearDownSuite() {
	if err := suite.pgContainer.Terminate(suite.ctx); err != nil {
		log.Fatalf("error terminating postgres container: %s", err)
	}
}

func (suite *RepoSuite) TestCreateMessage() {
	t := suite.T()

	log.Println(t)
}

func TestRepoSuite(t *testing.T) {
	suite.Run(t, new(RepoSuite))
}
