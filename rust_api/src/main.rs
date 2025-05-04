use axum::{routing::post, Json, Router};
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let port = 3000;
    let app = Router::new()
    .route("/submit", post(handle_post))
    .route("/point", post(handle_post_point));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("🚀 Server running at http://{}", addr);

    axum::serve(
        TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}

#[derive(Debug, Deserialize)]
struct FormData {
    username: String,
    email: String,
}

async fn handle_post(Json(payload): Json<FormData>) -> String {
    println!("{:?}", payload);
    format!("✅ Received: {} <{}>", payload.username, payload.email)
}

#[derive(Debug, Deserialize)]
struct Point{
    x: i32,
    y: i32,
}
async fn handle_post_point(Json(payload): Json<Point>) -> String {
    println!("{:?}", payload);
    format!("✅ Received: ({}, {})", payload.x, payload.y)
}