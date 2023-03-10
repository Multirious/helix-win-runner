#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("window not found")]
    WindowNotFound,
    #[error("atleast one of `window_title` or `window_process_name` argument must provided")]
    IncompleteSearchArgument,
}

pub type Result<T> = std::result::Result<T, Error>;
