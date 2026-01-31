use async_trait::async_trait;

use crate::domain::entities::todo::Todo;

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn create(&self, title: String) -> Result<Todo, sqlx::Error>;
    async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error>;
    async fn get_by_id(&self, id: u32) -> Result<Option<Todo>, sqlx::Error>;
    async fn update(
        &self,
        id: u32,
        title: Option<String>,
        completed: Option<bool>,
    ) -> Result<Option<Todo>, sqlx::Error>;
    async fn delete(&self, id: u32) -> Result<bool, sqlx::Error>;
    async fn reorder(&self, todo_ids: Vec<i64>) -> Result<(), sqlx::Error>;
}
