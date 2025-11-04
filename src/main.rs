use axum::{response::Html, routing::get, Router};
use maud::{html, Markup};
use tower_http::services::ServeDir;

const PORT_NUM: &str = "0.0.0.0:3000";
const DATASTAR_SRC: &str = "https://cdn.jsdelivr.net/gh/starfederation/datastar@1.0.0-RC.6/bundles/datastar.js";

fn app() -> Router {
    Router::new()
        .route("/", get(handler))
        .nest_service("/static", ServeDir::new("static"))}

#[tokio::main]
async fn main() {
    let listener: tokio::net::TcpListener = 
        tokio::net::TcpListener::bind(PORT_NUM).await.unwrap();
    axum::serve(listener, app()).await.unwrap(); }

fn greet() -> Markup {
    html! { h1 class="text-3xl font-bold underline" { "Hello!" } } }

fn explanation() -> Markup {
    html! { p { "This page is rendered within rust." } } }

const ALERT_MESSAGE: &str = "I’m sorry, Dave. I’m afraid I can’t do that.";

fn alert_button() -> Markup {
    html! {
        button
            type="button"
            data-on:click=(ALERT_MESSAGE)
        { "Open the pod bay doors, HAL." } } }

fn fido () -> Markup {
    html! {
        div {
            input
                data-bind:fidoval {}
            button
                type="button"
                data-class:success="$fidoval != ''"
                style="display: none;" {
                    "Save!" } } } }

async fn handler() -> Html<String> {
    let markup: Markup = html! {
        html {
            head { 
                script type="module" src=(DATASTAR_SRC) {}
                link rel="stylesheet" href="/static/output.css" {} }
            body {
                div {
                    (greet())
                    (fido())
                    (explanation())
                    (alert_button()) } } } };
    Html(markup.into_string()) }