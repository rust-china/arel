use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error("generate sql error info")]
    Message(String),
}
