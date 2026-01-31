use crate::application::ports::todo_repository::TodoRepository;
use crate::application::errors::AppError;

pub async fn execute(repo: &dyn TodoRepository, ids: Vec<i64>) -> Result<(), AppError> {
    repo.reorder(ids).await
}
