use rust_todo_app::create_app;

#[tokio::main]
async fn main() {
    let app = create_app("sqlite:todos.db").await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await.expect("Server error");
}
