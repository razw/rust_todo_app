pub mod handlers;
pub mod models;
pub mod store;

use axum::Router;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use store::TodoStore;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

// テスト用のアプリケーションを作成する関数
pub async fn create_test_app() -> Router {
    // メモリ内データベースを使用
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    // テーブルを作成
    sqlx::query(
        r#"
        CREATE TABLE todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0,
            position INTEGER NOT NULL DEFAULT 0,
        )
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    let store = TodoStore::new(pool);

    // ルーターを作成（main.rsから関数をインポート）
    create_router(store)
}

// 本番用のアプリケーションを作成する関数
pub async fn create_app(database_url: &str) -> Router {
    let connect_options = SqliteConnectOptions::from_str(database_url)
        .expect("Invalid database URL")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
        .expect("Failed to connect to database");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    let store = TodoStore::new(pool);
    create_router(store)
}

// ルーターを作成する共通関数
fn create_router(store: TodoStore) -> Router {
    use crate::handlers::*;
    use axum::{
        routing::{delete, get, post, put},
        Router,
    };

    // CORS設定
    let cors = CorsLayer::new()
        .allow_origin(Any) // 開発用: すべてのオリジンを許可。本番では .allow_origin(["http://localhost:3000"].map(|o| o.parse().unwrap())) などに変更
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    // ログ設定（HTTPリクエスト/レスポンスを自動ログ）
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &axum::http::Request<_>| {
            tracing::info_span!(
                "http_request",
                method = %request.method(),
                uri = %request.uri(),
            )
        })
        .on_request(|_request: &axum::http::Request<_>, _span: &tracing::Span| {
            tracing::debug!("request started");
        })
        .on_response(
            |_response: &axum::http::Response<_>,
             latency: std::time::Duration,
             _span: &tracing::Span| {
                tracing::info!(
                    latency = ?latency,
                    status = %_response.status(),
                    "request completed"
                );
            },
        )
        .on_failure(
            |_failure_class: ServerErrorsFailureClass,
             latency: std::time::Duration,
             _span: &tracing::Span| {
                tracing::error!(
                    failure = ?_failure_class,
                    latency = ?latency,
                    "request failed"
                );
            },
        );

    Router::new()
        .route("/", get(handler))
        .route("/todos", get(get_todos))
        .route("/todos", post(create_todo))
        .route("/todos/reorder", put(reorder_todos))
        .route("/todos/:id", get(get_todo_by_id))
        .route("/todos/:id", put(update_todo))
        .route("/todos/:id", delete(delete_todo))
        .with_state(store)
        .layer(cors)
        .layer(trace_layer)
}
