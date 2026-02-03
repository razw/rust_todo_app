use crate::application::errors::AppError;
use crate::application::ports::todo_repository::TodoRepository;

pub async fn execute(repo: &dyn TodoRepository, id: u32) -> Result<bool, AppError> {
    repo.delete(id).await
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
        result: bool,
    }

    #[async_trait]
    impl TodoRepository for FakeRepo {
        async fn create(&self, _title: String) -> Result<Todo, AppError> {
            unimplemented!("not needed for this test");
        }

        async fn get_all(&self) -> Result<Vec<Todo>, AppError> {
            unimplemented!("not needed for this test");
        }

        async fn get_by_id(&self, _id: u32) -> Result<Option<Todo>, AppError> {
            unimplemented!("not needed for this test");
        }

        async fn update(
            &self,
            _id: u32,
            _title: Option<String>,
            _completed: Option<bool>,
        ) -> Result<Option<Todo>, AppError> {
            unimplemented!("not needed for this test");
        }

        async fn delete(&self, id: u32) -> Result<bool, AppError> {
            *self
                .last_id
                .lock()
                .expect("failed to lock last_id") = Some(id);
            Ok(self.result)
        }

        async fn reorder(&self, _todo_ids: Vec<i64>) -> Result<(), AppError> {
            unimplemented!("not needed for this test");
        }
    }

    #[tokio::test]
    async fn delete_delegates_to_repository() {
        let repo = FakeRepo {
            last_id: Mutex::new(None),
            result: true,
        };

        let result = execute(&repo, 9).await.unwrap();

        let last_id = *repo.last_id.lock().expect("failed to lock last_id");
        assert_eq!(last_id, Some(9));
        assert!(result);
    }
}
