use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use axum::{body::Bytes, Router, response::{Html, IntoResponse}, routing::get, extract::{Extension, Query}};
use tower_http::add_extension::AddExtensionLayer;
use serde::{Deserialize, Serialize};
pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Default)]
pub struct AppState {
    pub db: HashMap<String, Bytes>,
}

pub struct NameQuery {
    pub name: String,
}
pub struct VisitorParams {
    pub name: String,
}
impl Default for VisitorParams {
    fn default() -> Self {
        Self {
            name: String::from("Unknown Visitor"),
        }
    }
}
    

async fn work_with_state(state: SharedState) {
    state
}

pub fn router(state: SharedState) -> Router<SharedState> {
    Router::with_state(state.clone())
        .route("/", get(hello_axum))
        .route("/hello", get(hello_handler) )
}

async fn hello_axum() -> &'static str {
    "<h1>Hello Axum</h1>"
}

async fn hello_handler(Query(names): Query<VisitorParams>) -> String {
    let name = names.params.name;
    format!("<h1>Hello {}</h1>", name)
}
