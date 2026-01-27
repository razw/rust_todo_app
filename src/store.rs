use std::sync::{Arc, Mutex};
use crate::models::Todo;

#[derive(Clone)]
pub struct TodoStore {
  todos: Arc<Mutex<Vec<Todo>>>,
  next_id: Arc<Mutex<u32>>,
}

impl TodoStore {
  pub fn new() -> Self {
    Self {
      todos: Arc::new(Mutex::new(Vec::new())),
      next_id: Arc::new(Mutex::new(1)),
    }
  }
  pub fn create(&self, title: String) -> Todo {
    let mut next_id = self.next_id.lock().unwrap();
    let id = *next_id;
    *next_id += 1;

    let todo = Todo {
      id,
      title,
      completed: false,
    };

    let mut todos = self.todos.lock().unwrap();
    todos.push(todo.clone());
    todo
  }
  pub fn get_all(&self) -> Vec<Todo> {
    let todos = self.todos.lock().unwrap();
    todos.clone()
  }
  pub fn get_by_id(&self, id: u32) -> Option<Todo> {
    let todos = self.todos.lock().unwrap();
    todos.iter().find(|todo| todo.id == id).cloned()
  }
  pub fn update(&self, id: u32, title: Option<String>, completed: Option<bool>) -> Option<Todo> {
    let mut todos = self.todos.lock().unwrap();
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
      if let Some(new_title) = title {
        todo.title = new_title;
      }
      if let Some(new_completed) = completed {
        todo.completed = new_completed;
      }
      Some(todo.clone())
    } else {
      None
    }
  }
  pub fn delete(&self, id: u32) -> bool {
    let mut todos = self.todos.lock().unwrap();
    if let Some(pos) = todos.iter().position(|todo| todo.id == id){
      todos.remove(pos);
      true
    } else {
      false
    }
  }
}
