docker build -f migrations.Dockerfile -t gogo-migration:dev .
docker build -f postgres.Dockerfile -t gogo-postgres:dev .
docker build -f gogoRestApp.Dockerfile -t gogo-rest-app:dev .