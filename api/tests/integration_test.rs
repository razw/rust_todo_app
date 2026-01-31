use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use rust_todo_app::create_test_app;
use tower::util::ServiceExt;

/// レスポンスボディをJSONとして取得するヘルパー
async fn response_json(response: axum::response::Response) -> serde_json::Value {
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&body_bytes).unwrap()
}

#[tokio::test]
async fn test_create_todo() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method("POST")
        .uri("/todos")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title": "テストTODO"}"#))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let todo = response_json(response).await;
    assert_eq!(todo["title"], "テストTODO");
    assert_eq!(todo["completed"], false);
    assert!(todo["id"].as_i64().unwrap() > 0);
}

#[tokio::test]
async fn test_get_todos() {
    let app = create_test_app().await;

    // TODOを2件作成
    for title in ["TODO 1", "TODO 2"] {
        let request = Request::builder()
            .method("POST")
            .uri("/todos")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::json!({"title": title}).to_string()))
            .unwrap();
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    // 一覧取得
    let request = Request::builder()
        .method("GET")
        .uri("/todos")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let todos: Vec<serde_json::Value> = response_json(response).await.as_array().unwrap().clone();
    assert_eq!(todos.len(), 2);
    assert_eq!(todos[0]["title"], "TODO 1");
    assert_eq!(todos[1]["title"], "TODO 2");
}

#[tokio::test]
async fn test_get_todo_by_id() {
    let app = create_test_app().await;

    // TODOを作成
    let create_request = Request::builder()
        .method("POST")
        .uri("/todos")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title": "取得テスト"}"#))
        .unwrap();
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let created = response_json(create_response).await;
    let id = created["id"].as_i64().unwrap();

    // IDで取得
    let request = Request::builder()
        .method("GET")
        .uri(format!("/todos/{}", id))
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let todo = response_json(response).await;
    assert_eq!(todo["id"], id);
    assert_eq!(todo["title"], "取得テスト");
}

#[tokio::test]
async fn test_get_todo_not_found() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method("GET")
        .uri("/todos/99999")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_todo() {
    let app = create_test_app().await;

    // TODOを作成
    let create_request = Request::builder()
        .method("POST")
        .uri("/todos")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title": "更新前"}"#))
        .unwrap();
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let created = response_json(create_response).await;
    let id = created["id"].as_i64().unwrap();

    // 更新
    let request = Request::builder()
        .method("PUT")
        .uri(format!("/todos/{}", id))
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"title": "更新後", "completed": true}"#.to_string(),
        ))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let updated = response_json(response).await;
    assert_eq!(updated["title"], "更新後");
    assert_eq!(updated["completed"], true);

    // 一覧で反映を確認
    let get_request = Request::builder()
        .method("GET")
        .uri(format!("/todos/{}", id))
        .body(Body::empty())
        .unwrap();
    let get_response = app.oneshot(get_request).await.unwrap();
    assert_eq!(get_response.status(), StatusCode::OK);
    let todo = response_json(get_response).await;
    assert_eq!(todo["title"], "更新後");
    assert_eq!(todo["completed"], true);
}

#[tokio::test]
async fn test_update_todo_not_found() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method("PUT")
        .uri("/todos/99999")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title": "存在しない"}"#))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_todo() {
    let app = create_test_app().await;

    // TODOを作成
    let create_request = Request::builder()
        .method("POST")
        .uri("/todos")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title": "削除テスト"}"#))
        .unwrap();
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let created = response_json(create_response).await;
    let id = created["id"].as_i64().unwrap();

    // 削除
    let request = Request::builder()
        .method("DELETE")
        .uri(format!("/todos/{}", id))
        .body(Body::empty())
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // 削除されたことを確認（404）
    let get_request = Request::builder()
        .method("GET")
        .uri(format!("/todos/{}", id))
        .body(Body::empty())
        .unwrap();
    let get_response = app.oneshot(get_request).await.unwrap();
    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_todo_not_found() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method("DELETE")
        .uri("/todos/99999")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_reorder_todos() {
    let app = create_test_app().await;

    // TODOを3件作成
    let mut ids = Vec::new();
    for title in ["一番目", "二番目", "三番目"] {
        let request = Request::builder()
            .method("POST")
            .uri("/todos")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::json!({"title": title}).to_string()))
            .unwrap();
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let created = response_json(response).await;
        ids.push(created["id"].as_i64().unwrap());
    }

    // 順序を逆に並べ替え（三番目, 二番目, 一番目）
    let reorder_request = Request::builder()
        .method("PUT")
        .uri("/todos/reorder")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({ "ids": [ids[2], ids[1], ids[0]] }).to_string(),
        ))
        .unwrap();
    let reorder_response = app.clone().oneshot(reorder_request).await.unwrap();
    assert_eq!(reorder_response.status(), StatusCode::OK);

    // 一覧取得して順序を確認
    let get_request = Request::builder()
        .method("GET")
        .uri("/todos")
        .body(Body::empty())
        .unwrap();
    let get_response = app.oneshot(get_request).await.unwrap();
    assert_eq!(get_response.status(), StatusCode::OK);

    let todos: Vec<serde_json::Value> = response_json(get_response)
        .await
        .as_array()
        .unwrap()
        .clone();
    assert_eq!(todos.len(), 3);
    assert_eq!(todos[0]["title"], "三番目");
    assert_eq!(todos[1]["title"], "二番目");
    assert_eq!(todos[2]["title"], "一番目");
}

#[tokio::test]
async fn test_reorder_todos_empty_ids() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method("PUT")
        .uri("/todos/reorder")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"ids": []}"#))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_validation_error_empty_title() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method("POST")
        .uri("/todos")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title": ""}"#))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let error = response_json(response).await;
    assert_eq!(error["error"], "Validation failed");
    assert!(error["details"].is_array());
}

#[tokio::test]
async fn test_validation_error_too_long_title() {
    let app = create_test_app().await;

    let long_title = "a".repeat(201);
    let request = Request::builder()
        .method("POST")
        .uri("/todos")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({ "title": long_title }).to_string(),
        ))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let error = response_json(response).await;
    assert_eq!(error["error"], "Validation failed");
    assert!(error["details"].is_array());
}

#[tokio::test]
async fn test_update_validation_error_empty_title() {
    let app = create_test_app().await;

    let create_request = Request::builder()
        .method("POST")
        .uri("/todos")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title": "更新用"}"#))
        .unwrap();
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    let created = response_json(create_response).await;
    let id = created["id"].as_i64().unwrap();

    let request = Request::builder()
        .method("PUT")
        .uri(format!("/todos/{}", id))
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title": ""}"#))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let error = response_json(response).await;
    assert_eq!(error["error"], "Validation failed");
    assert!(error["details"].is_array());
}

#[tokio::test]
async fn test_handler_hello() {
    let app = create_test_app().await;

    let request = Request::builder()
        .method("GET")
        .uri("/")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body = std::str::from_utf8(&body_bytes).unwrap();
    assert_eq!(body, "Hello, World!");
}
