use sqlx::FromRow;

use crate::domain::entities::todo::Todo;

#[derive(Debug, Clone, FromRow)]
pub struct DbTodo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub position: i64,
}

impl From<DbTodo> for Todo {
    fn from(row: DbTodo) -> Self {
        Self {
            id: row.id,
            title: row.title,
            completed: row.completed,
            position: row.position,
        }
    }
}
