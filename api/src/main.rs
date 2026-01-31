use rust_todo_app::create_app;

#[tokio::main]
async fn main() {
    // ログの初期化
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_todo_app=debug,tower_http=debug".into()),
        )
        .init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:todos.db".to_string());
    let app = create_app(&database_url).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("Server error");
}
