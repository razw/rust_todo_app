use crate::application::errors::AppError;
use crate::application::ports::todo_repository::TodoRepository;

pub async fn execute(repo: &dyn TodoRepository, ids: Vec<i64>) -> Result<(), AppError> {
    repo.reorder(ids).await
}
