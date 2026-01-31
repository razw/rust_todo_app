use crate::domain::entities::todo::Todo;
use crate::application::ports::todo_repository::TodoRepository;

pub async fn execute(
    repo: &dyn TodoRepository,
    id: u32,
    title: Option<String>,
    completed: Option<bool>,
) -> Result<Option<Todo>, sqlx::Error> {
    repo.update(id, title, completed).await
}
