mod constants;
mod components;
mod models;
mod pages;

use dotenvy::dotenv;
use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use sqlx::{postgres::PgPoolOptions};
use std::{env, time::Duration};
use constants::{PORT_NUM};
use models::AppState;

fn app(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(pages::landing::handler))
        .route("/product", get(pages::product::handler)) 
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