variable "postgres_image" {
  type        = string
  description = "Postgres image name"
}

variable "postgres_image_version" {
  type        = string
  description = "Postgres docker image tag version"
}

variable "postgres_port" {
  type      = number
  description = "Postgres port"
}

variable "postgres_user" {
  type    = string
  description = "Postgres root username"
}

variable "postgres_password" {
  type    = string
  description = "Postgres root password"
}

variable "kubernetes_namespace" {
  type        = string
  description = "Target Kubernetes namespace"
}
