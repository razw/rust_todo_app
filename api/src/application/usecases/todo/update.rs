use crate::application::errors::AppError;
use crate::application::ports::todo_repository::TodoRepository;
use crate::domain::entities::todo::Todo;

pub async fn execute(
    repo: &dyn TodoRepository,
    id: u32,
    title: Option<String>,
    completed: Option<bool>,
) -> Result<Option<Todo>, AppError> {
    repo.update(id, title, completed).await
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
        last_args: Mutex<Option<(u32, Option<String>, Option<bool>)>>,
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

        async fn get_by_id(&self, _id: u32) -> Result<Option<Todo>, AppError> {
            unimplemented!("not needed for this test");
        }

        async fn update(
            &self,
            id: u32,
            title: Option<String>,
            completed: Option<bool>,
        ) -> Result<Option<Todo>, AppError> {
            *self.last_args.lock().expect("failed to lock last_args") =
                Some((id, title, completed));
            Ok(self.todo.clone())
        }

        async fn delete(&self, _id: u32) -> Result<bool, AppError> {
            unimplemented!("not needed for this test");
        }

        async fn reorder(&self, _todo_ids: Vec<i64>) -> Result<(), AppError> {
            unimplemented!("not needed for this test");
        }
    }

    #[tokio::test]
    async fn update_delegates_to_repository() {
        let repo = FakeRepo {
            last_args: Mutex::new(None),
            todo: Some(Todo {
                id: 5,
                title: "updated".to_string(),
                completed: true,
                position: 2,
            }),
        };

        let result = execute(&repo, 5, Some("updated".to_string()), Some(true))
            .await
            .unwrap();

        let last_args = repo
            .last_args
            .lock()
            .expect("failed to lock last_args")
            .clone();
        assert_eq!(
            last_args,
            Some((5, Some("updated".to_string()), Some(true)))
        );
        assert_eq!(result.unwrap().title, "updated");
    }
}
