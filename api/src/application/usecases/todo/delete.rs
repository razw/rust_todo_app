use crate::application::errors::AppError;
use crate::application::ports::todo_repository::TodoRepository;

pub async fn execute(repo: &dyn TodoRepository, id: u32) -> Result<bool, AppError> {
    repo.delete(id).await
}
