use crate::application::errors::AppError;
use crate::application::ports::todo_repository::TodoRepository;
use crate::domain::entities::todo::Todo;

pub async fn execute(repo: &dyn TodoRepository) -> Result<Vec<Todo>, AppError> {
    repo.get_all().await
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
        called: Mutex<bool>,
        todos: Vec<Todo>,
    }

    #[async_trait]
    impl TodoRepository for FakeRepo {
        async fn create(&self, _title: String) -> Result<Todo, AppError> {
            unimplemented!("not needed for this test");
        }

        async fn get_all(&self) -> Result<Vec<Todo>, AppError> {
            *self.called.lock().expect("failed to lock called") = true;
            Ok(self.todos.clone())
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

        async fn reorder(&self, _todo_ids: Vec<i64>) -> Result<(), AppError> {
            unimplemented!("not needed for this test");
        }
    }

    #[tokio::test]
    async fn list_delegates_to_repository() {
        let repo = FakeRepo {
            called: Mutex::new(false),
            todos: vec![Todo {
                id: 1,
                title: "first".to_string(),
                completed: false,
                position: 1,
            }],
        };

        let result = execute(&repo).await.unwrap();

        let called = *repo.called.lock().expect("failed to lock called");
        assert!(called);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].title, "first");
    }
}
