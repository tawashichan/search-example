use std::{net::SocketAddr, sync::Arc};

use axum::{response::IntoResponse, routing::get, AddExtensionLayer, Json, Router, Server};
use domain::{
    interfaces::advertisement_repository::ArcIAdvertisementRepository,
    service::search_service::SearchService,
};
use infra::advertisement_repository::AdvertisementRepository;
use serde::Serialize;

struct Repositories {
    advertisement_repository: ArcIAdvertisementRepository,
}

#[derive(Clone)]
struct Services {
    search_service: SearchService,
}

fn init_services(repos: Repositories) -> Services {
    Services {
        search_service: SearchService::new(repos.advertisement_repository),
    }
}

pub async fn init_server() {
    let repos = Repositories {
        advertisement_repository: Arc::new(AdvertisementRepository::new()),
    };
    let services = init_services(repos);

    let app = Router::new()
        .route("/health_check", get(health_check))
        .layer(AddExtensionLayer::new(services));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Serialize)]
pub struct HealthCheckResp {
    pub result: String,
}

async fn health_check() -> impl IntoResponse {
    Json(HealthCheckResp {
        result: "ok".to_owned(),
    })
}

#[tokio::main]
async fn main() {
    init_server().await;
}
