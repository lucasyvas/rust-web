use thiserror::Error;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("'{0}' already in collection")]
    Conflict(Uuid),
    #[error("'{0}' not found in collection")]
    NotFound(Uuid),
    #[error("sqlx error")]
    Sqlx(#[from] sqlx::Error),
}
