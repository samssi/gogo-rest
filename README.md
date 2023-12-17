# gogo-rest
Basic REST API developed with Go and Gin framework

The API is simple:
    1. User can send messages to API with POST /v1/messages/
    2. User can pop the oldest message from the API with GET /v1/messages/

Endpoints are tested using testify-library with "local integration tests", meaning
that on setup the tests empty the local test database and insert fixtures to the empty database.
This prevents possible side effects uncleaned database might cause.

Database connection is done with Database Pool using pgx library.

## Running the software

Software can be run locally by setting up the local test database with docker-compose:

```
docker compose up -d gogo-postgres
```

main.go, main-function runs the software.

## Running the tests

During the tests Postgres-container must be run by using docker-compose:

```
docker compose up -d gogo-postgres
```

Then the test suites can be run from the command line and they will connect to the local db
```
go test ./test/...
```

(Testcontainer implementation could be a good idea to make the tests one command only and directly runnable.)

## Running the software in Kubernetes

The software has a local setup for running it in Mac OS with Colima.

Setting up Colima for Kubernetes (Apple Silicon)

```
colima start --arch aarch64 --cpu 4 --memory 16 --disk 100 --kubernetes
```

After Colima is running build the local docker images using local build script:
```
./docker-build-all.sh
```

Running services up to local Colima Kubernetes cluster:

```
cd terraform/gogo-rest/environments/dev/
terraform init
terraform apply
```

## Flyway migrations

Initial idea was to do also Flyway migrations to Kubernetes cluster. This option is still there but remains unused as
the DB schema is generated for local db when building the local DB Docker image.