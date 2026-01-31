use async_trait::async_trait;

use crate::application::errors::AppError;
use crate::application::ports::todo_repository::TodoRepository;
use crate::domain::entities::todo::Todo;
use crate::infrastructure::persistence::db_todo::DbTodo;
use sqlx::sqlite::SqlitePool;

#[derive(Clone)]
pub struct TodoStore {
    pool: SqlitePool,
}

impl TodoStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    async fn create_inner(&self, title: String) -> Result<Todo, AppError> {
        // 最大positionを取得
        let max_position: Option<i64> = sqlx::query_scalar("SELECT MAX(position) FROM todos")
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        let new_position = max_position.unwrap_or(0) + 1;

        // SQLiteではRETURNING句が使えないので、INSERT後に取得
        let result = sqlx::query("INSERT INTO todos (title, completed, position) VALUES (?, ?, ?)")
            .bind(&title)
            .bind(false)
            .bind(new_position)
            .execute(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        // 最後に挿入されたIDを取得
        let id = result.last_insert_rowid();

        Ok(Todo {
            id,
            title,
            completed: false,
            position: new_position,
        })
    }

    async fn get_all_inner(&self) -> Result<Vec<Todo>, AppError> {
        let rows = sqlx::query_as::<_, DbTodo>(
            "SELECT id, title, completed, position FROM todos ORDER BY position ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn get_by_id_inner(&self, id: u32) -> Result<Option<Todo>, AppError> {
        let row = sqlx::query_as::<_, DbTodo>(
            "SELECT id, title, completed, position FROM todos WHERE id = ?",
        )
        .bind(id as i64)
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(row.map(Into::into))
    }

    async fn update_inner(
        &self,
        id: u32,
        title: Option<String>,
        completed: Option<bool>,
    ) -> Result<Option<Todo>, AppError> {
        let mut todo = match self.get_by_id_inner(id).await? {
            Some(t) => t,
            None => return Ok(None),
        };

        if let Some(new_title) = title {
            todo.title = new_title;
        }
        if let Some(new_completed) = completed {
            todo.completed = new_completed;
        }

        sqlx::query("UPDATE todos SET title = ?, completed = ? WHERE id = ?")
            .bind(&todo.title)
            .bind(todo.completed)
            .bind(id as i64)
            .execute(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(Some(todo))
    }

    async fn delete_inner(&self, id: u32) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id as i64)
            .execute(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(result.rows_affected() > 0)
    }

    async fn reorder_inner(&self, todo_ids: Vec<i64>) -> Result<(), AppError> {
        for (index, id) in todo_ids.iter().enumerate() {
            sqlx::query("UPDATE todos SET position = ? WHERE id = ?")
                .bind(index as i64)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(map_sqlx_error)?;
        }
        Ok(())
    }
}

#[async_trait]
impl TodoRepository for TodoStore {
    async fn create(&self, title: String) -> Result<Todo, AppError> {
        self.create_inner(title).await
    }

    async fn get_all(&self) -> Result<Vec<Todo>, AppError> {
        self.get_all_inner().await
    }

    async fn get_by_id(&self, id: u32) -> Result<Option<Todo>, AppError> {
        self.get_by_id_inner(id).await
    }

    async fn update(
        &self,
        id: u32,
        title: Option<String>,
        completed: Option<bool>,
    ) -> Result<Option<Todo>, AppError> {
        self.update_inner(id, title, completed).await
    }

    async fn delete(&self, id: u32) -> Result<bool, AppError> {
        self.delete_inner(id).await
    }

    async fn reorder(&self, todo_ids: Vec<i64>) -> Result<(), AppError> {
        self.reorder_inner(todo_ids).await
    }
}

fn map_sqlx_error(error: sqlx::Error) -> AppError {
    AppError::unexpected(error.to_string())
}
