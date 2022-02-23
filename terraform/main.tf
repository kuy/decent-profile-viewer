terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "4.11.0"
    }
  }
}

provider "google" {
  credentials = file("~/.config/gcp-sa/cropd-prj-8995ab21aa87.json")

  project = "cropd-prj"
  region  = "us-central1"
  zone    = "us-central1-c"
}

resource "google_cloud_run_service" "profile-viewer-api" {
  name     = "profile-viewer-api"
  location = "us-central1"

  template {
    spec {
      containers {
        image = "gcr.io/cropd-prj/profile-viewer-api:v3"
        ports {
          container_port = 3000
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
