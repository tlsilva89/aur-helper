#[derive(Debug, Clone)]
pub enum AppError {
    CommandFailed(String),
    ParseError(String),
    NetworkError(String),
    IoError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::CommandFailed(m) => write!(f, "Command failed: {m}"),
            AppError::ParseError(m) => write!(f, "Parse error: {m}"),
            AppError::NetworkError(m) => write!(f, "Network error: {m}"),
            AppError::IoError(m) => write!(f, "IO error: {m}"),
        }
    }
}
