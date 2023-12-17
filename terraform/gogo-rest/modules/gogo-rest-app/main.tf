locals {
  gogo_rest_app_deployment_name    = "${var.gogo_rest_app_image}-deployment"
  gogo_rest_app_service_name       = "${var.gogo_rest_app_image}-service"
}

resource "kubernetes_deployment" "gogo-rest-app-deployment" {
  metadata {
    name = local.gogo_rest_app_deployment_name
    namespace = var.kubernetes_namespace
    labels = {
      app = local.gogo_rest_app_deployment_name
    }
  }

  spec {
    selector {
      match_labels = {
        app = local.gogo_rest_app_deployment_name
      }
    }
    replicas = var.replicas

    template {
      metadata {
        labels = {
          app = local.gogo_rest_app_deployment_name
        }
      }
      spec {
        container {
          name = var.gogo_rest_app_image
          image = "${var.gogo_rest_app_image}:${var.gogo_rest_app_image_version}"
          port {
            container_port = var.gogo_rest_app_port
          }
          env {
            name = "db_connection_string"
            value = "postgres://gogo:gogo@gogo-postgres-service:5432/gogo"
          }
        }
      }
    }
  }
}

resource "kubernetes_service" "gogo-rest-app-service" {
  metadata {
    name = local.gogo_rest_app_service_name
    namespace = var.kubernetes_namespace
  }
  spec {
    selector = {
      app = var.gogo_rest_app_image
    }
    port {
      port        = var.gogo_rest_app_public_port
      target_port = var.gogo_rest_app_port
      protocol    = "TCP"
    }
    type = "LoadBalancer"
  }
}