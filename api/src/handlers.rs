use crate::models::Todo;
use crate::store::TodoStore;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use tracing::{error, info, warn};
use validator::Validate;

// リクエスト構造体もhandlers.rsに移動
#[derive(Deserialize, Validate)]
pub struct CreateTodoRequest {
    #[validate(length(
        min = 1,
        max = 200,
        message = "タイトルは1文字以上200文字以下である必要があります"
    ))]
    pub title: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateTodoRequest {
    #[validate(length(
        min = 1,
        max = 200,
        message = "タイトルは1文字以上200文字以下である必要があります"
    ))]
    pub title: Option<String>,
    pub completed: Option<bool>,
}

pub async fn handler() -> &'static str {
    "Hello, World!"
}

pub async fn get_todos(State(store): State<TodoStore>) -> Json<Vec<Todo>> {
    info!("GET /todos: fetching all todos");
    let todos = store.get_all().await.unwrap();
    info!("GET /todos: returned {} todo(s)", todos.len());
    Json(todos)
}

pub async fn get_todo_by_id(
    State(store): State<TodoStore>,
    Path(id): Path<u32>,
) -> Result<Json<Todo>, StatusCode> {
    info!("GET /todos/{}: fetching todo by id", id);
    match store.get_by_id(id).await {
        Ok(Some(todo)) => {
            info!("GET /todos/{}: todo found", id);
            Ok(Json(todo))
        }
        Ok(None) => {
            warn!("GET /todos/{}: todo not found", id);
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("GET /todos/{}: database error: {:?}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_todo(
    State(store): State<TodoStore>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<Json<Todo>, (StatusCode, Json<serde_json::Value>)> {
    info!("POST /todos: creating todo with title: {}", payload.title);
    if let Err(errors) = payload.validate() {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(_, errors)| {
                errors.iter().map(|e| {
                    e.message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| "Invalid value".to_string())
                })
            })
            .collect();
        warn!("POST /todos: validation failed: {:?}", error_messages);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
            "error": "Validation failed",
            "details": error_messages
            })),
        ));
    }
    match store.create(payload.title).await {
        Ok(todo) => {
            info!("POST /todos: todo created successfully, id={}", todo.id);
            Ok(Json(todo))
        }
        Err(e) => {
            error!("POST /todos: failed to create todo: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to create todo"
                })),
            ))
        }
    }
}

pub async fn update_todo(
    State(store): State<TodoStore>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<Todo>, (StatusCode, Json<serde_json::Value>)> {
    info!("PUT /todos/{}: updating todo", id);
    if let Err(errors) = payload.validate() {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(_, errors)| {
                errors.iter().map(|e| {
                    e.message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| "Invalid value".to_string())
                })
            })
            .collect();
        warn!("PUT /todos/{}: validation failed: {:?}", id, error_messages);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
            "error": "Validation failed",
            "details": error_messages
            })),
        ));
    }
    match store.update(id, payload.title, payload.completed).await {
        Ok(Some(todo)) => {
            info!("PUT /todos/{}: todo updated successfully", id);
            Ok(Json(todo))
        }
        Ok(None) => {
            warn!("PUT /todos/{}: todo not found", id);
            Err((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": "Todo not found"
                })),
            ))
        }
        Err(e) => {
            error!("PUT /todos/{}: database error: {:?}", id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to update todo"
                })),
            ))
        }
    }
}

pub async fn delete_todo(
    State(store): State<TodoStore>,
    Path(id): Path<u32>,
) -> Result<StatusCode, StatusCode> {
    info!("DELETE /todos/{}: deleting todo", id);
    match store.delete(id).await {
        Ok(true) => {
            info!("DELETE /todos/{}: todo deleted successfully", id);
            Ok(StatusCode::NO_CONTENT)
        }
        Ok(false) => {
            warn!("DELETE /todos/{}: todo not found", id);
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("DELETE /todos/{}: database error: {:?}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
