use crate::domain::entities::todo::Todo;
use crate::application::ports::todo_repository::TodoRepository;

pub async fn execute(repo: &dyn TodoRepository, title: String) -> Result<Todo, sqlx::Error> {
    repo.create(title).await
}
