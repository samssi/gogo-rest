locals {
  postgres_schema         = "gogo"
  postgres_host           = "gogo-postgres-service"
  postgres_port           = 5432
  postgres_user           = "gogo"
  postgres_password       = "gogo"
  postgres_connection_url = "${local.postgres_user}:${local.postgres_password}@${local.postgres_host}:${local.postgres_port}/${local.postgres_schema}"
  postgres_image          = "gogo-postgres"
  postgres_image_version  = "dev"
  kubernetes_namespace    = "gogo"
}

terraform {
  required_version = "~> 1.6.6"

  required_providers {
    kubernetes = {
      source = "hashicorp/kubernetes"
      version = "2.24.0"
    }
  }
}

provider "kubernetes" {
  config_path = "~/.kube/config"
  config_context = "colima"
}

module "kubernetes-config" {
  source = "../../modules/kubernetes-config"
}

module "postgres" {
  source  = "../../modules/postgres"
  kubernetes_namespace = local.kubernetes_namespace
  postgres_image = local.postgres_image
  postgres_image_version = local.postgres_image_version
  postgres_port = local.postgres_port
  postgres_user = local.postgres_user
  postgres_password = local.postgres_password
}

module "gogo-rest-app" {
  source = "../../modules/gogo-rest-app"
  gogo_rest_app_image = "gogo-rest-app"
  gogo_rest_app_image_version = "dev"
  gogo_rest_app_port = "8080"
  gogo_rest_app_public_port = "8085"
  kubernetes_namespace = "gogo"
  replicas = 2
}