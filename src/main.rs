use axum::{routing::get, Router, Json};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Message {
    message: &'static str,
}

async fn root() -> Json<Message> {
    Json(Message { message: "Hello World" })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    // bind to an address and serve using axum::serve
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind");

    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("server failed");
}
