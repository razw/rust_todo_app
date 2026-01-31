use serde::{Deserialize};
use validator::Validate;

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

#[derive(Deserialize)]
pub struct ReorderRequest {
    pub ids: Vec<i64>,
}
