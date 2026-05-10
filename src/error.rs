use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum CodeseedError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Conflict {
        path: PathBuf,
        message: String,
    },
}

impl CodeseedError {
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }

    pub fn conflict(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self::Conflict {
            path: path.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for CodeseedError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(formatter, "{}: {}", path.display(), source)
            }
            Self::Conflict { path, message } => {
                write!(formatter, "{}: {}", path.display(), message)
            }
        }
    }
}

impl std::error::Error for CodeseedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Conflict { .. } => None,
        }
    }
}

pub type Result<T> = std::result::Result<T, CodeseedError>;
