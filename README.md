# gogo-rest
Basic CRUD REST API developed with Go

## Dev instructions

Running Postgres using docker-compose:

```
docker-compose up -d
```

Setting up Colima for Kubernetes (Apple Silicon)

```
colima start --arch aarch64 --cpu 4 --memory 16 --disk 100 --kubernetes
```

Deleting colima environment

```
colima delete
```

Running Postgres using Kubernetes:

```
cd terraform/gogo-rest/environments/dev/
terraform init
terraform apply
```