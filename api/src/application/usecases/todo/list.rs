use crate::domain::entities::todo::Todo;
use crate::application::ports::todo_repository::TodoRepository;

pub async fn execute(repo: &dyn TodoRepository) -> Result<Vec<Todo>, sqlx::Error> {
    repo.get_all().await
}
