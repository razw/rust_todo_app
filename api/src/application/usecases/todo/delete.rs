use crate::application::ports::todo_repository::TodoRepository;
use crate::application::errors::AppError;

pub async fn execute(repo: &dyn TodoRepository, id: u32) -> Result<bool, AppError> {
    repo.delete(id).await
}
