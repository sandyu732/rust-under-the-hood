#![allow(unused)] // TODO: delete this line once the lab is done
//! Lab 10.4 — The Lab 10.3 API backed by SQLite with sqlx.
//!
//!   cargo run -p lab-10-4-sqlx-todos                 # uses ./todos.db
//!   DATABASE_URL=sqlite::memory: cargo run -p lab-10-4-sqlx-todos
//!
//! The HTTP contract is IDENTICAL to Lab 10.3 and the assessment is the same
//! test suite. Only the state changes: `SqlitePool` instead of a HashMap.
//! Queries use runtime-checked `sqlx::query_as::<_, Row>(...)`. Stretch goal:
//! switch to compile-time-checked `sqlx::query_as!` (needs `DATABASE_URL` at
//! build time or `cargo sqlx prepare`).

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub done: bool,
}

/// How a row comes back from SQLite (ids are stored as TEXT).
#[derive(Debug, sqlx::FromRow)]
struct Row {
    id: String,
    title: String,
    done: bool,
}

impl From<Row> for Todo {
    fn from(r: Row) -> Self {
        Todo {
            id: Uuid::parse_str(&r.id).expect("ids are written by us"),
            title: r.title,
            done: r.done,
        }
    }
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

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    Validation(String),
    Database(sqlx::Error),
}

/// `?` on a sqlx call inside a handler converts the error automatically.
impl From<sqlx::Error> for ApiError {
    fn from(e: sqlx::Error) -> Self {
        ApiError::Database(e)
    }
}

/// As Lab 10.3, plus Database → 500 `{"error":"internal error"}` (log the
/// real error with `tracing::error!`, never leak it to the client).
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        todo!("lab-10-4-sqlx-todos: implement me")
    }
}

/// Open the pool and create the table if needed.
/// For `sqlite::memory:` use a single connection (each connection would
/// otherwise get its own empty in-memory database).
pub async fn connect(url: &str) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(url)?.create_if_missing(true);
    let max = if url.contains(":memory:") { 1 } else { 5 };
    let pool = SqlitePoolOptions::new()
        .max_connections(max)
        .connect_with(options)
        .await?;
    // TODO: run a CREATE TABLE IF NOT EXISTS todos (id TEXT PRIMARY KEY, title TEXT NOT NULL, done BOOLEAN NOT NULL)
    Ok(pool)
}

/// Given: same routes as Lab 10.3.
pub fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/{id}",
            get(get_todo).patch(update_todo).delete(delete_todo),
        )
        .with_state(pool)
}

pub async fn list_todos(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>, ApiError> {
    todo!("lab-10-4-sqlx-todos: implement me")
}

pub async fn create_todo(
    State(pool): State<SqlitePool>,
    Json(input): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), ApiError> {
    todo!("lab-10-4-sqlx-todos: implement me")
}

pub async fn get_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>, ApiError> {
    todo!("lab-10-4-sqlx-todos: implement me")
}

pub async fn update_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTodo>,
) -> Result<Json<Todo>, ApiError> {
    todo!("lab-10-4-sqlx-todos: implement me")
}

pub async fn delete_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    todo!("lab-10-4-sqlx-todos: implement me")
}
