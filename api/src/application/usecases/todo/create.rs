use crate::application::ports::todo_repository::TodoRepository;
use crate::domain::entities::todo::Todo;
use crate::application::errors::AppError;

pub async fn execute(repo: &dyn TodoRepository, title: String) -> Result<Todo, AppError> {
    repo.create(title).await
}
