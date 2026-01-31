use crate::presentation::dto::todo_requests::{
    CreateTodoRequest, ReorderRequest, UpdateTodoRequest,
};
use crate::presentation::dto::todo_responses::TodoResponse;
use std::sync::Arc;

use crate::application::errors::AppError;
use crate::application::ports::todo_repository::TodoRepository;
use crate::application::usecases::todo::{
    create as create_todo, delete as delete_todo_usecase, get as get_todo, list as list_todos,
    reorder as reorder_todos_usecase, update as update_todo_usecase,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tracing::{error, info, warn};
use validator::Validate;

pub async fn handler() -> &'static str {
    "Hello, World!"
}

pub async fn get_todos(State(repo): State<Arc<dyn TodoRepository>>) -> Json<Vec<TodoResponse>> {
    info!("GET /todos: fetching all todos");
    let todos = list_todos::execute(repo.as_ref()).await.unwrap();
    info!("GET /todos: returned {} todo(s)", todos.len());
    let responses: Vec<TodoResponse> = todos.into_iter().map(Into::into).collect();
    Json(responses)
}

pub async fn get_todo_by_id(
    State(repo): State<Arc<dyn TodoRepository>>,
    Path(id): Path<u32>,
) -> Result<Json<TodoResponse>, StatusCode> {
    info!("GET /todos/{}: fetching todo by id", id);
    match get_todo::execute(repo.as_ref(), id).await {
        Ok(Some(todo)) => {
            info!("GET /todos/{}: todo found", id);
            Ok(Json(todo.into()))
        }
        Ok(None) => {
            warn!("GET /todos/{}: todo not found", id);
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("GET /todos/{}: repository error: {:?}", id, e);
            Err(app_error_status(&e))
        }
    }
}

pub async fn create_todo(
    State(repo): State<Arc<dyn TodoRepository>>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<Json<TodoResponse>, (StatusCode, Json<serde_json::Value>)> {
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
    match create_todo::execute(repo.as_ref(), payload.title).await {
        Ok(todo) => {
            info!("POST /todos: todo created successfully, id={}", todo.id);
            Ok(Json(todo.into()))
        }
        Err(e) => {
            error!("POST /todos: failed to create todo: {:?}", e);
            Err(app_error_response(&e))
        }
    }
}

pub async fn update_todo(
    State(repo): State<Arc<dyn TodoRepository>>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<TodoResponse>, (StatusCode, Json<serde_json::Value>)> {
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
    match update_todo_usecase::execute(repo.as_ref(), id, payload.title, payload.completed).await {
        Ok(Some(todo)) => {
            info!("PUT /todos/{}: todo updated successfully", id);
            Ok(Json(todo.into()))
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
            error!("PUT /todos/{}: repository error: {:?}", id, e);
            Err(app_error_response(&e))
        }
    }
}

pub async fn delete_todo(
    State(repo): State<Arc<dyn TodoRepository>>,
    Path(id): Path<u32>,
) -> Result<StatusCode, StatusCode> {
    info!("DELETE /todos/{}: deleting todo", id);
    match delete_todo_usecase::execute(repo.as_ref(), id).await {
        Ok(true) => {
            info!("DELETE /todos/{}: todo deleted successfully", id);
            Ok(StatusCode::NO_CONTENT)
        }
        Ok(false) => {
            warn!("DELETE /todos/{}: todo not found", id);
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("DELETE /todos/{}: repository error: {:?}", id, e);
            Err(app_error_status(&e))
        }
    }
}

pub async fn reorder_todos(
    State(repo): State<Arc<dyn TodoRepository>>,
    Json(payload): Json<ReorderRequest>,
) -> Result<StatusCode, StatusCode> {
    info!("PUT /todos/reorder: reordering todos");
    match reorder_todos_usecase::execute(repo.as_ref(), payload.ids).await {
        Ok(_) => {
            info!("PUT /todos/reorder: todos reordered successfully");
            Ok(StatusCode::OK)
        }
        Err(e) => {
            error!("PUT /todos/reorder: repository error: {:?}", e);
            Err(app_error_status(&e))
        }
    }
}

fn app_error_status(error: &AppError) -> StatusCode {
    match error {
        AppError::Validation(_) => StatusCode::BAD_REQUEST,
        AppError::NotFound => StatusCode::NOT_FOUND,
        AppError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn app_error_response(error: &AppError) -> (StatusCode, Json<serde_json::Value>) {
    match error {
        AppError::Validation(message) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Validation failed",
                "details": [message],
            })),
        ),
        AppError::NotFound => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Todo not found",
            })),
        ),
        AppError::Unexpected(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Unexpected error",
            })),
        ),
    }
}
