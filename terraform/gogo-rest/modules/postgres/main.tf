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
        app = var.postgres_image
      }
    }
    replicas = 1

    template {
      metadata {
        labels = {
          app = var.postgres_image
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
      app = var.postgres_image
    }
    port {
      port = var.postgres_port
      target_port = var.postgres_port
      protocol = "TCP"
    }
  }
}