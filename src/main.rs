use axum::{routing::get, Router};

const PORT_NUM: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let app: axum::Router<_> = 
        Router::new().route("/", get(|| async {"Hello World!"}));
    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind(PORT_NUM).await.unwrap();
    axum::serve(listener, app).await.unwrap();}
