use crate::html::HtmlError;

use crate::file::FileError;
use std::error;
use std::fmt::{self, Debug, Display, Formatter};
use std::io;

// Definitions -----------------------------------------------------------

#[derive(Debug)]
pub enum ErrorKind {
    File,
    Html,
}

#[derive(Debug)]
pub enum SystemError {
    Local { kind: ErrorKind, message: String },
    Internal { source: io::Error },
}

// std Impl -----------------------------------------------------------

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::File => write!(f, "file"),
            Self::Html => write!(f, "html"),
        }
    }
}

impl Display for SystemError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal { source } => write!(f, "[Internal] error: {}", source),
            Self::Local { kind, message } => {
                write!(f, "{} error: {}", kind.to_string(), message)
            }
        }
    }
}

impl error::Error for SystemError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Internal { source } => Some(source),
            _ => None,
        }
    }
}

impl From<io::Error> for SystemError {
    fn from(source: io::Error) -> Self {
        Self::Internal { source }
    }
}

impl From<HtmlError> for SystemError {
    fn from(e: HtmlError) -> Self {
        match e {
            HtmlError::Internal { source } => Self::Internal { source },
            _ => Self::Local {
                kind: ErrorKind::Html,
                message: e.to_string(),
            },
        }
    }
}

impl From<FileError> for SystemError {
    fn from(e: FileError) -> Self {
        match e {
            FileError::Internal { source } => Self::Internal { source },
        }
    }
}
