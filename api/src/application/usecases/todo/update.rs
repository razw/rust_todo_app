use crate::application::errors::AppError;
use crate::application::ports::todo_repository::TodoRepository;
use crate::domain::entities::todo::Todo;

pub async fn execute(
    repo: &dyn TodoRepository,
    id: u32,
    title: Option<String>,
    completed: Option<bool>,
) -> Result<Option<Todo>, AppError> {
    repo.update(id, title, completed).await
}
