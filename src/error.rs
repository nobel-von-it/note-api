#[derive(Debug, thiserror::Error)]
pub enum NoteApiError {
    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Ls error: {0}")]
    LsError(#[from] NoteApiLsError),
}

#[derive(Debug, thiserror::Error)]
pub enum NoteApiLsError {
    #[error("File not found. Path: {0}")]
    FileNotFound(String),
    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),
}

pub type NoteApiResult<T> = Result<T, NoteApiError>;

pub type NoteApiLsResult<T> = Result<T, NoteApiLsError>;
