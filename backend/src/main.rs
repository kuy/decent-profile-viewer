use axum::{
    extract::{Path, Query},
    routing::{get, Router},
};
use std::{collections::HashMap, net::SocketAddr};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/profiles/:id", get(profile));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ping() -> &'static str {
    "pong"
}

async fn profile(Path(id): Path<String>, Query(params): Query<HashMap<String, String>>) -> String {
    tracing::debug!("params={:?}", params);

    let endpoint =
        std::env::var("VISUALIZER_ENDPOINT").unwrap_or_else(|_| "http://localhost:18080".into());
    let url = format!("{}/api/shots/{}/profile", endpoint, id);
    tracing::debug!("url={}", url);

    let res = reqwest::get(url).await;
    match res {
        Ok(res) => res.text().await.unwrap(),
        Err(e) => panic!("ERROR: {:?}", e),
    }
}
