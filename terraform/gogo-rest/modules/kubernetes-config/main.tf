resource "kubernetes_namespace" "gogo" {
  metadata {
    name = "gogo"
  }
}