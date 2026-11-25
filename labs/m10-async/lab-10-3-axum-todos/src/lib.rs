#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 10.3 — REST API with axum.
//!
//!   cargo run -p lab-10-3-axum-todos
//!   curl -s localhost:3000/todos -H 'content-type: application/json' -d '{"title":"learn rust"}'
//!
//! | Method | Path          | Success                | Errors          |
//! |--------|---------------|------------------------|-----------------|
//! | GET    | /health       | 200 "ok"               |                 |
//! | GET    | /todos        | 200 [Todo] by title    |                 |
//! | POST   | /todos        | 201 Todo               | 422 empty title |
//! | GET    | /todos/{id}   | 200 Todo               | 404             |
//! | PATCH  | /todos/{id}   | 200 Todo               | 404, 422        |
//! | DELETE | /todos/{id}   | 204                    | 404             |
//!
//! Errors are JSON: `{"error": "<message>"}`. Handlers return
//! `Result<_, ApiError>` and `ApiError: IntoResponse` turns errors into HTTP.

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub done: Option<bool>,
}

pub type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    Validation(String),
}

/// NotFound → 404 `{"error":"not found"}`; Validation(msg) → 422 `{"error": msg}`.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        todo!("lab-10-3-axum-todos: implement me")
    }
}

/// Given: the router. Your job is the handlers.
pub fn app(db: Db) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/{id}",
            get(get_todo).patch(update_todo).delete(delete_todo),
        )
        .with_state(db)
}

/// All todos, sorted by title.
pub async fn list_todos(State(db): State<Db>) -> Json<Vec<Todo>> {
    todo!("lab-10-3-axum-todos: implement me")
}

/// Title is trimmed; empty after trimming → Validation("title must not be empty").
pub async fn create_todo(
    State(db): State<Db>,
    Json(input): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), ApiError> {
    todo!("lab-10-3-axum-todos: implement me")
}

pub async fn get_todo(State(db): State<Db>, Path(id): Path<Uuid>) -> Result<Json<Todo>, ApiError> {
    todo!("lab-10-3-axum-todos: implement me")
}

/// Only the fields present are changed. Same title validation as create.
pub async fn update_todo(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTodo>,
) -> Result<Json<Todo>, ApiError> {
    todo!("lab-10-3-axum-todos: implement me")
}

pub async fn delete_todo(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    todo!("lab-10-3-axum-todos: implement me")
}
