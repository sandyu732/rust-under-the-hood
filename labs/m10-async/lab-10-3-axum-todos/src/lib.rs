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
        // SOLUTION-START
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "not found".to_string()),
            ApiError::Validation(m) => (StatusCode::UNPROCESSABLE_ENTITY, m),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
        // SOLUTION-END
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

// SOLUTION-ONLY-START
fn validate_title(title: &str) -> Result<String, ApiError> {
    let t = title.trim();
    if t.is_empty() {
        Err(ApiError::Validation("title must not be empty".into()))
    } else {
        Ok(t.to_string())
    }
}
// SOLUTION-ONLY-END

/// All todos, sorted by title.
pub async fn list_todos(State(db): State<Db>) -> Json<Vec<Todo>> {
    // SOLUTION-START
    let mut todos: Vec<Todo> = db.read().await.values().cloned().collect();
    todos.sort_by(|a, b| a.title.cmp(&b.title).then(a.id.cmp(&b.id)));
    Json(todos)
    // SOLUTION-END
}

/// Title is trimmed; empty after trimming → Validation("title must not be empty").
pub async fn create_todo(
    State(db): State<Db>,
    Json(input): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), ApiError> {
    // SOLUTION-START
    let todo = Todo {
        id: Uuid::new_v4(),
        title: validate_title(&input.title)?,
        done: false,
    };
    db.write().await.insert(todo.id, todo.clone());
    Ok((StatusCode::CREATED, Json(todo)))
    // SOLUTION-END
}

pub async fn get_todo(State(db): State<Db>, Path(id): Path<Uuid>) -> Result<Json<Todo>, ApiError> {
    // SOLUTION-START
    db.read()
        .await
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(ApiError::NotFound)
    // SOLUTION-END
}

/// Only the fields present are changed. Same title validation as create.
pub async fn update_todo(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTodo>,
) -> Result<Json<Todo>, ApiError> {
    // SOLUTION-START
    let mut todos = db.write().await;
    let todo = todos.get_mut(&id).ok_or(ApiError::NotFound)?;
    if let Some(title) = input.title {
        todo.title = validate_title(&title)?;
    }
    if let Some(done) = input.done {
        todo.done = done;
    }
    Ok(Json(todo.clone()))
    // SOLUTION-END
}

pub async fn delete_todo(
    State(db): State<Db>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    // SOLUTION-START
    db.write()
        .await
        .remove(&id)
        .map(|_| StatusCode::NO_CONTENT)
        .ok_or(ApiError::NotFound)
    // SOLUTION-END
}
