use sqlx::sqlite::SqlitePool;
use crate::models::Todo;

#[derive(Clone)]
pub struct TodoStore {
  pool: SqlitePool,
}

impl TodoStore {
  pub fn new(pool: SqlitePool) -> Self {
    Self { pool }
  }

  pub async fn create(&self, title: String) -> Result<Todo, sqlx::Error> {
    // SQLiteではRETURNING句が使えないので、INSERT後に取得
    let result = sqlx::query(
      "INSERT INTO todos (title, completed) VALUES (?, ?)",
    )
    .bind(&title)
    .bind(false)
    .execute(&self.pool)
    .await?;

    // 最後に挿入されたIDを取得
    let id = result.last_insert_rowid();

    Ok(Todo {
      id,
      title,
      completed: false,
    })
  }

  pub async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as::<_, Todo>(
      "SELECT id, title, completed FROM todos"
    )
    .fetch_all(&self.pool)
    .await?;

    Ok(todos)
  }

  pub async fn get_by_id(&self, id: u32) -> Result<Option<Todo>, sqlx::Error> {
    let todo = sqlx::query_as::<_, Todo>(
      "SELECT id, title, completed FROM todos WHERE id = ?"
    )
    .bind(id as i64)
    .fetch_optional(&self.pool)
    .await?;

    Ok(todo)
  }

  pub async fn update(
    &self,
    id: u32,
    title: Option<String>,
    completed: Option<bool>
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

    sqlx::query(
      "UPDATE todos SET title = ?, completed = ? WHERE id = ?"
    )
    .bind(&todo.title)
    .bind(todo.completed)
    .bind(id as i64)
    .execute(&self.pool)
    .await?;

    Ok(Some(todo))
  }

  pub async fn delete(&self, id: u32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
      "DELETE FROM todos WHERE id = ?"
    )
    .bind(id as i64)
    .execute(&self.pool)
    .await?;

    Ok(result.rows_affected() > 0)
  }
}
