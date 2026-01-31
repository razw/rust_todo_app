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

    pub async fn create(&self, title: String) -> Result<Todo, sqlx::Error> {
        // 最大positionを取得
        let max_position: Option<i64> = sqlx::query_scalar("SELECT MAX(position) FROM todos")
            .fetch_one(&self.pool)
            .await?;

        let new_position = max_position.unwrap_or(0) + 1;

        // SQLiteではRETURNING句が使えないので、INSERT後に取得
        let result = sqlx::query("INSERT INTO todos (title, completed, position) VALUES (?, ?, ?)")
            .bind(&title)
            .bind(false)
            .bind(new_position)
            .execute(&self.pool)
            .await?;

        // 最後に挿入されたIDを取得
        let id = result.last_insert_rowid();

        Ok(Todo {
            id,
            title,
            completed: false,
            position: new_position,
        })
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error> {
        let rows = sqlx::query_as::<_, DbTodo>(
            "SELECT id, title, completed, position FROM todos ORDER BY position ASC",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    pub async fn get_by_id(&self, id: u32) -> Result<Option<Todo>, sqlx::Error> {
        let row = sqlx::query_as::<_, DbTodo>(
            "SELECT id, title, completed, position FROM todos WHERE id = ?",
        )
        .bind(id as i64)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Into::into))
    }

    pub async fn update(
        &self,
        id: u32,
        title: Option<String>,
        completed: Option<bool>,
    ) -> Result<Option<Todo>, sqlx::Error> {
        let mut todo = match self.get_by_id(id).await? {
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
            .await?;

        Ok(Some(todo))
    }

    pub async fn delete(&self, id: u32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id as i64)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn reorder(&self, todo_ids: Vec<i64>) -> Result<(), sqlx::Error> {
        for (index, id) in todo_ids.iter().enumerate() {
            sqlx::query("UPDATE todos SET position = ? WHERE id = ?")
                .bind(index as i64)
                .bind(id)
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }
}
