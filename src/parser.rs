use crate::html::{HtmlAttribute, HtmlDocument, HtmlError, HtmlParser};

use select::predicate::Predicate;

use std::error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::str::Utf8Error;

// Definitions -----------------------------------------------------------

#[derive(Debug)]
pub enum ParserError<E> {
    Failed(E),
    Internal { source: io::Error },
    Utf8 { source: Utf8Error },
}

pub struct Utils;

pub struct Parser<T> {
    parse: T,
}

// Parser Impl -----------------------------------------------------------

impl<T> Parser<T> {
    pub fn new(kind: T) -> Self {
        Self { parse: kind }
    }
}

impl Parser<Utils> {
    fn fix_link(link: &str) -> &str {
        link.trim_start_matches("https")
            .trim_start_matches("http")
            .trim_start_matches(":")
            .trim_start_matches("//")
            .trim_start_matches("www.")
            .trim_end_matches("/")
    }
}

// Trait Impl ---------------------------------------------------------

impl<T> HtmlParser for Parser<T>
where
    T: HtmlDocument,
{
    fn links<P: Predicate>(&self, predicate: P) -> Result<Vec<String>, ParserError<HtmlError>> {
        match self.parse.document() {
            Ok(doc) => Ok(doc
                .find(predicate)
                .filter_map(|n| {
                    if let Some(link) = n.attr(HtmlAttribute::Href.into()) {
                        Some(Parser::<Utils>::fix_link(link))
                    } else {
                        None
                    }
                })
                .map(|link| link.to_string())
                .collect()),
            Err(err) => Err(ParserError::from(err)),
        }
    }
}

// std Impl -----------------------------------------------------------

impl<E> Display for ParserError<E>
where
    E: Display + error::Error,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failed(err) => write!(f, "failed to parse | {}", err.to_string()),
            Self::Internal { source } => write!(f, "failed to parse | {}", source),
            Self::Utf8 { source } => write!(f, "failed to parse | {}", source),
        }
    }
}

impl From<HtmlError> for ParserError<HtmlError> {
    fn from(error: HtmlError) -> Self {
        match error {
            HtmlError::Internal { source } => Self::Internal { source },
            HtmlError::Utf8 { source } => Self::Utf8 { source },
            _ => Self::Failed(error),
        }
    }
}

impl<E> From<io::Error> for ParserError<E> {
    fn from(source: io::Error) -> Self {
        Self::Internal { source }
    }
}

impl<E> From<Utf8Error> for ParserError<E> {
    fn from(source: Utf8Error) -> Self {
        Self::Utf8 { source }
    }
}
