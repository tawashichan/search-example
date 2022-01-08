terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "3.5.0"
    }
  }

  // gcsへのアクセスの際に使用するcredentialは環境変数経由で渡す必要がある
  // providerで指定しても使われないので注意
  backend "gcs" {
    bucket = "gcp-practice-api-server-hoge"
    prefix = "terraform/state" // what?
  }
}

provider "google" {
  credentials = file("./gcp-practice-336816-bb04ffa0e352.json")

  project = "gcp-practice-336816"
  region  = "asia-northeast1"
  zone    = "asia-northeast1-a"
}

resource "google_artifact_registry_repository" "api-server-repo" {
  provider = google-beta

  project       = "gcp-practice-336816"
  location      = "asia-northeast1"
  repository_id = "api-server-repo"
  description   = "example docker repository"
  format        = "DOCKER"
}

/*resource "google_cloud_run_service" "api-server" {
  name     = "api-server"
  location = "asia-northeast1-a"

  template {
    spec {
      containers {
        image = "us-docker.pkg.dev/cloudrun/container/hello"
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }
}*/
