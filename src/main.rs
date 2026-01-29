use rust_todo_app::create_app;
use tracing_subscriber;

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

    let app = create_app("sqlite:todos.db").await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");
    
    tracing::info!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.expect("Server error");
}
