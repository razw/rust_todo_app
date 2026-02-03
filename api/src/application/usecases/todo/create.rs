use crate::application::errors::AppError;
use crate::application::ports::todo_repository::TodoRepository;
use crate::domain::entities::todo::Todo;

pub async fn execute(repo: &dyn TodoRepository, title: String) -> Result<Todo, AppError> {
    repo.create(title).await
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
        last_title: Mutex<Option<String>>,
        todo: Todo,
    }

    #[async_trait]
    impl TodoRepository for FakeRepo {
        async fn create(&self, title: String) -> Result<Todo, AppError> {
            *self.last_title.lock().expect("failed to lock last_title") = Some(title);
            Ok(self.todo.clone())
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

        async fn reorder(&self, _todo_ids: Vec<i64>) -> Result<(), AppError> {
            unimplemented!("not needed for this test");
        }
    }

    #[tokio::test]
    async fn create_delegates_to_repository() {
        let repo = FakeRepo {
            last_title: Mutex::new(None),
            todo: Todo {
                id: 1,
                title: "saved".to_string(),
                completed: false,
                position: 1,
            },
        };

        let result = execute(&repo, "write tests".to_string()).await;

        let stored_title = repo
            .last_title
            .lock()
            .expect("failed to lock last_title")
            .clone();
        assert_eq!(stored_title, Some("write tests".to_string()));
        assert_eq!(result.unwrap().title, "saved");
    }
}
