#[derive(Debug, Clone)]
pub enum AppError {
    NotFound,
    Validation(String),
    Unexpected(String),
}

impl AppError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    pub fn unexpected(message: impl Into<String>) -> Self {
        Self::Unexpected(message.into())
    }
}
