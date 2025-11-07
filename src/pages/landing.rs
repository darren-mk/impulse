use axum::{extract::State, response::Html};
use maud::{html, Markup};
use crate::models::AppState;
use crate::components::layouts::to_html;

pub async fn handler(State(state): State<AppState>) -> Html<String> {
    let total: i64 = sqlx::query_scalar("select increment_visit()")
        .fetch_one(&state.pool).await
        .expect("increment_visit failed");
    let body: Markup = html! {
        div {
            (greet(total))
            (fido())
            (explanation())
            (alert_button()) } };
    Html(to_html(body).into_string()) }

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
            
const ALERT_MESSAGE: &str = 
    "I’m sorry, Dave. I’m afraid I can’t do that.";

fn greet(total: i64) -> Markup {
    html! { 
        h1 class="text-3xl font-bold underline" { "Hello!" }
        h2 { (total) } } }

fn explanation() -> Markup {
    html! { p { "This page is rendered within rust." } } }