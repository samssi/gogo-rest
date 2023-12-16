locals {
  flyway_image            = "gogo-migration"
  flyway_image_version    = "dev"
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
  config_path    = "~/.kube/config"
  config_context = "colima"
}

module "migrations" {
  source = "../../modules/migrations"
  flyway_db_user = "gogo"
  flyway_db_password = "gogo"
  flyway_name = "flyway"
  flyway_image = local.flyway_image
  flyway_image_version = local.flyway_image_version
  kubernetes_namespace = "gogo"
}