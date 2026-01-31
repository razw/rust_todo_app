use crate::domain::entities::todo::Todo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TodoResponse {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub position: i64,
}

impl From<Todo> for TodoResponse {
    fn from(todo: Todo) -> Self {
        Self {
            id: todo.id,
            title: todo.title,
            completed: todo.completed,
            position: todo.position,
        }
    }
}
