use crate::domain::entities::todo::Todo;
use crate::application::ports::todo_repository::TodoRepository;

pub async fn execute(repo: &dyn TodoRepository, id: u32) -> Result<Option<Todo>, sqlx::Error> {
    repo.get_by_id(id).await
}
