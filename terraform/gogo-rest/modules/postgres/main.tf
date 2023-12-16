locals {
  postgres_deployment_name    = "${var.postgres_image}-deployment"
  postgres_service_name       = "${var.postgres_image}-service"
}

resource "kubernetes_deployment" "gogo-postgres-deployment" {
  metadata {
    name   = local.postgres_deployment_name
    namespace = var.kubernetes_namespace
  }
  spec {
    selector {
      match_labels = {
        app = local.postgres_deployment_name
      }
    }
    replicas = 1

    template {
      metadata {
        labels = {
          app = local.postgres_deployment_name
        }
      }

      spec {
        container {
          name  = var.postgres_image
          image = "${var.postgres_image}:${var.postgres_image_version}"
          port {
            container_port = var.postgres_port
          }
          env {
            name  = "POSTGRES_PASSWORD"
            value = var.postgres_password
          }
          env {
            name = "POSTGRES_USER"
            value = var.postgres_password
          }
        }
      }
    }
  }
}

resource "kubernetes_service" "gogo-postgres-service" {
  metadata {
    name = local.postgres_service_name
    namespace = var.kubernetes_namespace
  }
  spec {
    selector = {
      app = local.postgres_service_name
    }
    port {
      port = var.postgres_port
      target_port = var.postgres_port
      protocol = "TCP"
    }
  }
}