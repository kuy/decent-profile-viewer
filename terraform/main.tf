terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "4.76.0"
    }
  }

  backend "gcs" {
    bucket = "cropd-tf-state"
    prefix = "prd"
  }
}

provider "google" {
  project = "cropd-prj"
  region  = "us-central1"
  zone    = "us-central1-c"
}

resource "google_storage_bucket" "tf-state" {
  name          = "cropd-tf-state"
  location      = "us-central1"
  storage_class = "REGIONAL"
}

variable "backend" {
  type = object({
    image_tag = string
    visualizer_endpoint = string
  })

  default = {
    image_tag : "v3"
    visualizer_endpoint = "http://localhost:18080"
  }
}

resource "google_cloud_run_service" "profile-viewer-api" {
  name     = "profile-viewer-api"
  location = "us-central1"

  template {
    spec {
      containers {
        image = "gcr.io/cropd-prj/profile-viewer-api:${var.backend.image_tag}"
        ports {
          container_port = 3000
        }
        env {
          name = "RUST_LOG"
          value = "debug"
        }
        env {
          name = "VISUALIZER_ENDPOINT"
          value = var.backend.visualizer_endpoint
        }
      }
      container_concurrency = 2
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }
}

data "google_iam_policy" "noauth" {
  binding {
    role = "roles/run.invoker"
    members = [
      "allUsers",
    ]
  }
}

resource "google_cloud_run_service_iam_policy" "noauth" {
  location = google_cloud_run_service.profile-viewer-api.location
  project  = google_cloud_run_service.profile-viewer-api.project
  service  = google_cloud_run_service.profile-viewer-api.name

  policy_data = data.google_iam_policy.noauth.policy_data
}
