// Copyright 2025 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache

use axum::{Json, Router, routing::get};
use steampick::App;

pub mod get_items;

#[tokio::main]
async fn main() {
  get_items::get_data().await;
  axum::serve(
    tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap(),
    Router::new().route("/", get(handler)),
  )
  .await
  .unwrap();
}

async fn handler() -> Json<Vec<App>> {
  Json(get_items::get_data().await.iter().map(|(_, v)| v.clone()).collect())
}
