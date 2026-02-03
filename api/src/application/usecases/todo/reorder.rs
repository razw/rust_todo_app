use crate::application::errors::AppError;
use crate::application::ports::todo_repository::TodoRepository;

pub async fn execute(repo: &dyn TodoRepository, ids: Vec<i64>) -> Result<(), AppError> {
    repo.reorder(ids).await
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
        last_ids: Mutex<Option<Vec<i64>>>,
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

        async fn delete(&self, _id: u32) -> Result<bool, AppError> {
            unimplemented!("not needed for this test");
        }

        async fn reorder(&self, todo_ids: Vec<i64>) -> Result<(), AppError> {
            *self.last_ids.lock().expect("failed to lock last_ids") = Some(todo_ids);
            Ok(())
        }
    }

    #[tokio::test]
    async fn reorder_delegates_to_repository() {
        let repo = FakeRepo {
            last_ids: Mutex::new(None),
        };

        let ids = vec![3, 1, 2];
        execute(&repo, ids.clone()).await.unwrap();

        let last_ids = repo
            .last_ids
            .lock()
            .expect("failed to lock last_ids")
            .clone();
        assert_eq!(last_ids, Some(ids));
    }
}
