# Capstone C: Production-grade service (spec)

Grow Lab 10.4 into something you'd be comfortable deploying. Create the crate yourself: `cargo new capstones/c-production-service/todo-service`, then add it to the root `Cargo.toml` `members` list.

## Requirements

1. **Postgres** via `sqlx` (`features = ["runtime-tokio", "postgres", "macros", "migrate"]`), with migrations in `migrations/` run at startup through `sqlx::migrate!()`. Use compile-time-checked queries (`query_as!`) and commit `.sqlx/` from `cargo sqlx prepare` so CI builds without a database.
2. **Configuration** from environment variables into a typed `Config` struct, failing fast with a clear error. Optionally layer a file over the environment (the `config` or `figment` crates).
3. **Observability:** `tracing` with JSON output in production and pretty output locally; a request-ID span per request (`tower-http`'s `TraceLayer`); a `/health` endpoint (process alive) and a `/ready` endpoint (database reachable).
4. **Graceful shutdown** on SIGTERM/Ctrl-C with `axum::serve(...).with_graceful_shutdown(...)`, letting in-flight requests finish.
5. **Errors:** one `ApiError` enum with `thiserror` + `IntoResponse`. Never leak internal error text to clients; log it instead.
6. **Tests:** handler tests with `tower::ServiceExt::oneshot`, and integration tests against a real Postgres (`#[sqlx::test]` gives each test a fresh database).
7. **Container:** a multi-stage `Dockerfile` (`rust` builder → `gcr.io/distroless/cc` or `scratch` with musl). Target an image under 30 MB.
8. **CI** (GitHub Actions): `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo audit`, and the Docker build. Cache with `Swatinem/rust-cache`.

## Done when

- [ ] `docker compose up` starts Postgres and the service; `curl localhost:3000/ready` returns 200
- [ ] CI is green on a PR
- [ ] p50/p99 latency measured under load (`oha` or `wrk`) and written in the README
- [ ] Killing the container mid-load drops no in-flight requests

## Reading

*Zero To Production in Rust* by Luca Palmieri walks through almost exactly this project.
