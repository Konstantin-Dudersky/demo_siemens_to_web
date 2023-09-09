use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum Errors {
    SqlxError(String),
}

impl From<SqlxError> for Errors {
    fn from(value: SqlxError) -> Self {
        Self::SqlxError(value.to_string())
    }
}
