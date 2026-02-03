#[derive(Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub position: i64,
}

#[cfg(test)]
mod tests {
    use super::Todo;

    #[test]
    fn todo_holds_given_fields() {
        let todo = Todo {
            id: 1,
            title: "write_tests".to_string(),
            completed: false,
            position: 10,
        };

        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "write_tests");
        assert!(!todo.completed);
        assert_eq!(todo.position, 10);
    }

    #[test]
    fn todo_is_coleneable() {
        let todo = Todo {
            id: 2,
            title: "clone me".to_string(),
            completed: true,
            position: 20,
        };

        let cloned = todo.clone();
        assert_eq!(cloned.id, 2);
        assert_eq!(cloned.title, "clone me");
        assert!(cloned.completed);
        assert_eq!(cloned.position, 20);
    }
}
