//! In-process HTTP tests: `oneshot` sends one request straight into the
//! Router — no network, no port.
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use lab_10_4_sqlx_todos::{app, connect};
use serde_json::{Value, json};
use sqlx::SqlitePool as Db;
use tower::ServiceExt;

async fn call(db: &Db, method: &str, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
    let mut req = Request::builder().method(method).uri(uri);
    let body = match body {
        Some(v) => {
            req = req.header("content-type", "application/json");
            Body::from(v.to_string())
        }
        None => Body::empty(),
    };
    let res = app(db.clone())
        .oneshot(req.body(body).unwrap())
        .await
        .unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let json = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, json)
}

#[tokio::test]
async fn crud_happy_path() {
    let db = connect("sqlite::memory:").await.unwrap();
    let (s, created) = call(
        &db,
        "POST",
        "/todos",
        Some(json!({"title": "  write tests "})),
    )
    .await;
    assert_eq!(s, StatusCode::CREATED);
    assert_eq!(created["title"], "write tests");
    assert_eq!(created["done"], false);
    let id = created["id"].as_str().unwrap().to_string();

    call(&db, "POST", "/todos", Some(json!({"title": "a first"}))).await;
    let (s, list) = call(&db, "GET", "/todos", None).await;
    assert_eq!(s, StatusCode::OK);
    let titles: Vec<&str> = list
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["title"].as_str().unwrap())
        .collect();
    assert_eq!(titles, ["a first", "write tests"]);

    let (s, updated) = call(
        &db,
        "PATCH",
        &format!("/todos/{id}"),
        Some(json!({"done": true})),
    )
    .await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(updated["done"], true);
    assert_eq!(updated["title"], "write tests");

    let (s, fetched) = call(&db, "GET", &format!("/todos/{id}"), None).await;
    assert_eq!((s, fetched), (StatusCode::OK, updated));

    let (s, _) = call(&db, "DELETE", &format!("/todos/{id}"), None).await;
    assert_eq!(s, StatusCode::NO_CONTENT);
    let (s, body) = call(&db, "GET", &format!("/todos/{id}"), None).await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    assert_eq!(body, json!({"error": "not found"}));
}

#[tokio::test]
async fn validation_and_bad_input() {
    let db = connect("sqlite::memory:").await.unwrap();
    let (s, body) = call(&db, "POST", "/todos", Some(json!({"title": "   "}))).await;
    assert_eq!(s, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body, json!({"error": "title must not be empty"}));

    let req = Request::post("/todos")
        .header("content-type", "application/json")
        .body(Body::from("{not json"))
        .unwrap();
    let res = app(db.clone()).oneshot(req).await.unwrap();
    assert_eq!(
        res.status(),
        StatusCode::BAD_REQUEST,
        "malformed JSON is rejected by the Json extractor"
    );

    let (s, _) = call(
        &db,
        "PATCH",
        "/todos/00000000-0000-0000-0000-000000000000",
        Some(json!({"done": true})),
    )
    .await;
    assert_eq!(s, StatusCode::NOT_FOUND);
    let (s, _) = call(
        &db,
        "DELETE",
        "/todos/00000000-0000-0000-0000-000000000000",
        None,
    )
    .await;
    assert_eq!(s, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn health() {
    let res = app(connect("sqlite::memory:").await.unwrap())
        .oneshot(Request::get("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn data_survives_a_new_router() {
    // The pool, not the Router, owns the data: a fresh Router sees old rows.
    let db = connect("sqlite::memory:").await.unwrap();
    call(&db, "POST", "/todos", Some(json!({"title": "persisted"}))).await;
    let (_, list) = call(&db, "GET", "/todos", None).await;
    assert_eq!(list.as_array().unwrap().len(), 1);
}
