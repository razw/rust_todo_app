use crate::application::errors::AppError;
use crate::application::ports::todo_repository::TodoRepository;
use crate::domain::entities::todo::Todo;

pub async fn execute(repo: &dyn TodoRepository, id: u32) -> Result<Option<Todo>, AppError> {
    repo.get_by_id(id).await
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;

    use super::execute;
    use crate::application::errors::AppError;
    use crate::application::ports::todo_repository::TodoRepository;
    use crate::domain::entities::todo::Todo;

    struct FakeRepo {
        last_id: Mutex<Option<u32>>,
        todo: Option<Todo>,
    }

    #[async_trait]
    impl TodoRepository for FakeRepo {
        async fn create(&self, _title: String) -> Result<Todo, AppError> {
            unimplemented!("not needed for this test");
        }

        async fn get_all(&self) -> Result<Vec<Todo>, AppError> {
            unimplemented!("not needed for this test");
        }

        async fn get_by_id(&self, id: u32) -> Result<Option<Todo>, AppError> {
            *self.last_id.lock().expect("failed to lock last_id") = Some(id);
            Ok(self.todo.clone())
        }

        async fn update(
            &self,
            _id: u32,
            _title: Option<String>,
            _completed: Option<bool>,
        ) -> Result<Option<Todo>, AppError> {
            unimplemented!("not needed for this test");
        }

        async fn delete(&self, _id: u32) -> Result<bool, AppError> {
            unimplemented!("not needed for this test");
        }

        async fn reorder(&self, _todo_ids: Vec<i64>) -> Result<(), AppError> {
            unimplemented!("not needed for this test");
        }
    }

    #[tokio::test]
    async fn get_delegates_to_repository() {
        let repo = FakeRepo {
            last_id: Mutex::new(None),
            todo: Some(Todo {
                id: 10,
                title: "stored".to_string(),
                completed: true,
                position: 3,
            }),
        };

        let result = execute(&repo, 42).await.unwrap();

        let last_id = *repo.last_id.lock().expect("failed to lock last_id");
        assert_eq!(last_id, Some(42));
        assert_eq!(result.unwrap().title, "stored");
    }
}
