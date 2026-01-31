use crate::application::errors::AppError;
use crate::application::ports::todo_repository::TodoRepository;
use crate::domain::entities::todo::Todo;

pub async fn execute(repo: &dyn TodoRepository, id: u32) -> Result<Option<Todo>, AppError> {
    repo.get_by_id(id).await
}
