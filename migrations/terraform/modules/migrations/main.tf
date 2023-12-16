resource "kubernetes_job" "gogo-migrations" {
  metadata {
    name = var.flyway_name
    namespace = var.kubernetes_namespace
  }
  spec {
    template {
      metadata {}
      spec {
        container {
          name = var.flyway_name
          image = "${var.flyway_image}:${var.flyway_image_version}"
          resources {
            limits = {
              cpu    = "2"
              memory = "2Gi"
            }
          }
          command = ["flyway", "-user=${var.flyway_db_user}", "-password=${var.flyway_db_password}", "-postgresql.transactional.lock=false", "-lockRetryCount=1", "-url=jdbc:postgresql://postgres-service:5432/gogo", "migrate"]
        }
        restart_policy = "Never"
      }
    }
    backoff_limit = 1
  }
  wait_for_completion = true
  timeouts {
    create = "2m"
    update = "2m"
  }
}