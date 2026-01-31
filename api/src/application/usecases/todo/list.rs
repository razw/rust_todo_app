use crate::application::ports::todo_repository::TodoRepository;
use crate::application::errors::AppError;
use crate::domain::entities::todo::Todo;

pub async fn execute(repo: &dyn TodoRepository) -> Result<Vec<Todo>, AppError> {
    repo.get_all().await
}
