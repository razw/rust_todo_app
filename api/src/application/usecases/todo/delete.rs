use crate::application::ports::todo_repository::TodoRepository;

pub async fn execute(repo: &dyn TodoRepository, id: u32) -> Result<bool, sqlx::Error> {
    repo.delete(id).await
}
