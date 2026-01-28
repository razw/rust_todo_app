pub mod models;
pub mod store;
pub mod handlers;

use axum::Router;
use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
use std::str::FromStr;
use store::TodoStore;
use crate::handlers::*;

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
            completed BOOLEAN NOT NULL DEFAULT 0
        )
        "#
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
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    let store = TodoStore::new(pool);
    create_router(store)
}

// ルーターを作成する共通関数
fn create_router(store: TodoStore) -> Router {
    use axum::{
        routing::{get, post, put, delete},
        Router,
    };
    use crate::handlers::*; // ハンドラー関数を別モジュールに移動する場合
    
    Router::new()
        .route("/", get(handler))
        .route("/todos", get(get_todos))
        .route("/todos", post(create_todo))
        .route("/todos/:id", get(get_todo_by_id))
        .route("/todos/:id", put(update_todo))
        .route("/todos/:id", delete(delete_todo))
        .with_state(store)
}
