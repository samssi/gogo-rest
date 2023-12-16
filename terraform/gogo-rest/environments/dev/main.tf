locals {
  postgres_host           = "gogo-postgres"
  postgres_port           = 5432
  postgres_user           = "gogo"
  postgres_password       = "gogo"
  postgres_secret_name    = "gogo-postgres-secret"
  postgres_image          = "postgres"
  postgres_image_version  = "16.1-alpine3.19"
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
  kubernetes_namespace = "gogo"
  replicas = 1
}