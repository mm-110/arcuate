//! Error type for output writing operations.

/// Errors that can occur while writing markdown output.
#[derive(Debug)]
pub enum OutputWriterError {
    IoError(std::io::Error),
}

impl std::fmt::Display for OutputWriterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputWriterError::IoError(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl std::error::Error for OutputWriterError {}

impl From<std::io::Error> for OutputWriterError {
    fn from(e: std::io::Error) -> Self {
        OutputWriterError::IoError(e)
    }
}
