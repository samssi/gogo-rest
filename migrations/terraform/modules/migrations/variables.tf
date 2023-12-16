variable "flyway_name" {
  type        = string
  description = "Flyway image name"
}

variable "flyway_image" {
  type        = string
  description = "Flyway image name"
}

variable "flyway_db_user" {
  type        = string
  description = "Flyway db username"
}

variable "flyway_db_password" {
  type        = string
  description = "Flyway db password"
}

variable "kubernetes_namespace" {
  type        = string
  description = "Target Kubernetes namespace"
}

variable "flyway_image_version" {
  type        = string
  description = "Image version of flyway container"
}