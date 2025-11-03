use axum::{response::Html, routing::get, Router};
use maud::{html, Markup};

const PORT_NUM: &str = "0.0.0.0:3000";
const DATASTAR_SRC: &str = "https://cdn.jsdelivr.net/gh/starfederation/datastar@1.0.0-RC.6/bundles/datastar.js";

#[tokio::main]
async fn main() {
    let app: axum::Router<_> = 
        Router::new().route("/", get(handler));
    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind(PORT_NUM).await.unwrap();
    axum::serve(listener, app).await.unwrap();}

fn greet() -> Markup {
    html! { h1 { "Hello!" } } }

fn explanation() -> Markup {
    html! { p { "This page is rendered within rust." } } }

async fn handler() -> Html<String> {
    let markup: Markup = html! {
        html {
            head { script type="module" src=(DATASTAR_SRC) {} }
            body {
                div {
                    (greet())  
                    (explanation()) } } } };
    Html(markup.into_string()) }