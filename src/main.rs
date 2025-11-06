mod constants;
mod models;

use dotenvy::dotenv;
use axum::{extract::State, response::Html, routing::get, Router};
use maud::{html, Markup};
use tower_http::services::ServeDir;
use sqlx::{postgres::PgPoolOptions};
use std::{env, time::Duration};
use constants::{PORT_NUM, DATASTAR_SRC};
use models::AppState;

fn app(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(handler)) 
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state) }

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url: String = 
        env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");
    let pool: sqlx::Pool<sqlx::Postgres> = 
        PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(3))
        .max_connections(5)
        .connect(&db_url).await.expect("Failed to create Postgres pool");       
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await.expect("Failed to connect to DB");
    let app_state = AppState { pool: pool.clone() };
    let listener: tokio::net::TcpListener = 
        tokio::net::TcpListener::bind(PORT_NUM).await.unwrap();
    axum::serve(listener, app(app_state)).await.unwrap(); }

fn greet(total: i64) -> Markup {
    html! { 
        h1 class="text-3xl font-bold underline" { "Hello!" }
        h2 { (total) } } }

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

async fn handler(State(state): State<AppState>) -> Html<String> {
    let total: i64 = sqlx::query_scalar("select increment_visit()")
        .fetch_one(&state.pool)
        .await
        .expect("increment_visit failed");
    let markup: Markup = html! {
        html {
            head { 
                script type="module" src=(DATASTAR_SRC) {}
                link rel="stylesheet" href="/static/output.css" {} }
            body {
                div {
                    (greet(total))
                    (fido())
                    (explanation())
                    (alert_button()) } } } };
    Html(markup.into_string()) }