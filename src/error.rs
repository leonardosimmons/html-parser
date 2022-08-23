use crate::file::FileError;
use crate::html::HtmlError;
use crate::parser::ParserError;

use std::error;
use std::fmt::{self, Debug, Display, Formatter};
use std::io;
use std::str::Utf8Error;

// Definitions --------------------------------------------------------

#[derive(Debug)]
pub enum ErrorKind {
    File,
    Html,
    Parser,
}

#[derive(Debug)]
pub enum SystemError {
    Local { kind: ErrorKind, message: String },
    Internal { source: io::Error },
    Utf8 { source: Utf8Error },
}

// std Impl -----------------------------------------------------------

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::File => write!(f, "file"),
            Self::Html => write!(f, "html"),
            Self::Parser => write!(f, "parser"),
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
            Self::Utf8 { source } => write!(f, "[Utf8] error: {}", source),
        }
    }
}

impl error::Error for SystemError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Internal { source } => Some(source),
            Self::Utf8 { source } => Some(source),
            _ => None,
        }
    }
}

impl From<io::Error> for SystemError {
    fn from(source: io::Error) -> Self {
        Self::Internal { source }
    }
}

impl From<Utf8Error> for SystemError {
    fn from(source: Utf8Error) -> Self {
        Self::Utf8 { source }
    }
}

impl From<HtmlError> for SystemError {
    fn from(error: HtmlError) -> Self {
        match error {
            HtmlError::Internal { source } => Self::Internal { source },
            HtmlError::Utf8 { source } => Self::Utf8 { source },
            _ => Self::Local {
                kind: ErrorKind::Html,
                message: error.to_string(),
            },
        }
    }
}

impl From<FileError> for SystemError {
    fn from(error: FileError) -> Self {
        match error {
            FileError::Internal { source } => Self::Internal { source },
        }
    }
}

impl<E> From<ParserError<E>> for SystemError
where
    E: Display + error::Error,
{
    fn from(error: ParserError<E>) -> Self {
        match error {
            ParserError::Internal { source } => Self::Internal { source },
            ParserError::Utf8 { source } => Self::Utf8 { source },
            _ => Self::Local {
                kind: ErrorKind::Parser,
                message: error.to_string(),
            },
        }
    }
}
