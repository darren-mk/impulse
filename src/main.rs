use askama::Template;
use axum::{response::Html, routing::get, Router};

const PORT_NUM: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let app: axum::Router<_> = 
        Router::new().route("/", get(handler));
    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind(PORT_NUM).await.unwrap();
    axum::serve(listener, app).await.unwrap();}

async fn handler() -> Html<String> {
    Html(IndexTemplate { body: "<h1>Hello</h1>" }.render().unwrap())}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> { body: &'a str, }
