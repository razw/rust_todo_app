use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
  pub id: u32,
  pub title: String,
  pub completed: bool,
}
