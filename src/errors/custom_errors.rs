use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("File not found: {0}")]
    DirectoryNotFound(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Unknown error occurred | Info: {0}")]
    Unknown(String),
}
