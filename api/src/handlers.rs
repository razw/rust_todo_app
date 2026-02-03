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

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use axum::body::{to_bytes, Body};
    use axum::http::{Request, StatusCode};
    use axum::routing::{delete, post};
    use axum::Router;
    use tower::ServiceExt;

    use super::{create_todo, delete_todo};
    use crate::application::errors::AppError;
    use crate::application::ports::todo_repository::TodoRepository;
    use crate::domain::entities::todo::Todo;

    struct FakeRepo {
        created_title: Mutex<Option<String>>,
        deleted_id: Mutex<Option<u32>>,
        create_result: Result<Todo, AppError>,
        delete_result: Result<bool, AppError>,
    }

    #[async_trait]
    impl TodoRepository for FakeRepo {
        async fn create(&self, title: String) -> Result<Todo, AppError> {
            *self
                .created_title
                .lock()
                .expect("failed to lock created_title") = Some(title);
            self.create_result.clone()
        }

        async fn get_all(&self) -> Result<Vec<Todo>, AppError> {
            unimplemented!("not needed for this test");
        }

        async fn get_by_id(&self, _id: u32) -> Result<Option<Todo>, AppError> {
            unimplemented!("not needed for this test");
        }

        async fn update(
            &self,
            _id: u32,
            _title: Option<String>,
            _completed: Option<bool>,
        ) -> Result<Option<Todo>, AppError> {
            unimplemented!("not needed for this test");
        }

        async fn delete(&self, id: u32) -> Result<bool, AppError> {
            *self.deleted_id.lock().expect("failed to lock deleted_id") = Some(id);
            self.delete_result.clone()
        }

        async fn reorder(&self, _todo_ids: Vec<i64>) -> Result<(), AppError> {
            unimplemented!("not needed for this test");
        }
    }

    fn app(repo: Arc<dyn TodoRepository>) -> Router {
        Router::new()
            .route("/todos", post(create_todo))
            .route("/todos/:id", delete(delete_todo))
            .with_state(repo)
    }

    #[tokio::test]
    async fn create_todo_returns_json() {
        let repo = Arc::new(FakeRepo {
            created_title: Mutex::new(None),
            deleted_id: Mutex::new(None),
            create_result: Ok(Todo {
                id: 1,
                title: "saved".to_string(),
                completed: false,
                position: 1,
            }),
            delete_result: Ok(false),
        });

        let response = app(repo.clone())
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/todos")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":"write tests"}"#))
                    .expect("failed to build request"),
            )
            .await
            .expect("request failed");

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("failed to read body");
        let json: serde_json::Value = serde_json::from_slice(&body).expect("failed to parse json");

        assert_eq!(json["title"], "saved");

        let created_title = repo
            .created_title
            .lock()
            .expect("failed to lock created_title")
            .clone();
        assert_eq!(created_title, Some("write tests".to_string()));
    }

    #[tokio::test]
    async fn create_todo_validation_error() {
        let repo = Arc::new(FakeRepo {
            created_title: Mutex::new(None),
            deleted_id: Mutex::new(None),
            create_result: Ok(Todo {
                id: 1,
                title: "saved".to_string(),
                completed: false,
                position: 1,
            }),
            delete_result: Ok(false),
        });

        let response = app(repo.clone())
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/todos")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"title":""}"#))
                    .expect("failed to build request"),
            )
            .await
            .expect("request failed");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("failed to read body");
        let json: serde_json::Value = serde_json::from_slice(&body).expect("failed to parse json");

        assert_eq!(json["error"], "Validation failed");
        let created_title = repo
            .created_title
            .lock()
            .expect("failed to lock created_title")
            .clone();
        assert!(created_title.is_none());
    }

    #[tokio::test]
    async fn delete_todo_returns_no_content() {
        let repo = Arc::new(FakeRepo {
            created_title: Mutex::new(None),
            deleted_id: Mutex::new(None),
            create_result: Ok(Todo {
                id: 1,
                title: "saved".to_string(),
                completed: false,
                position: 1,
            }),
            delete_result: Ok(true),
        });

        let response = app(repo.clone())
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/todos/42")
                    .body(Body::empty())
                    .expect("failed to build request"),
            )
            .await
            .expect("request failed");

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        let deleted_id = *repo.deleted_id.lock().expect("failed to lock deleted_id");
        assert_eq!(deleted_id, Some(42));
    }
}
