variable "replicas" {
  type = number
  description = "Number of replicas for the software to run"
}

variable "gogo_rest_app_image" {
  type        = string
  description = "Gogo rest app image name"
}

variable "gogo_rest_app_image_version" {
  type        = string
  description = "Gogo rest app image name"
}

variable "gogo_rest_app_port" {
  type        = number
  description = "Gogo rest app port"
}

variable "kubernetes_namespace" {
  type        = string
  description = "Target Kubernetes namespace"
}