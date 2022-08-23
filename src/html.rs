use crate::parser::ParserError;

use bytes::Bytes;
use select::document::Document;
use select::predicate::Predicate;

use std::error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::str::{self, FromStr, Utf8Error};
use std::sync::{Arc, Mutex};

// Traits --------------------------------------------------------------

pub trait HtmlDocument {
    fn bytes(&self) -> Bytes;
    fn document(&self) -> Result<Document, HtmlError>;
    fn text(&self) -> Result<String, HtmlError>;
}

pub trait HtmlParser {
    fn links<P: Predicate>(&self, p: P) -> Result<Vec<String>, ParserError<HtmlError>>;
}

// Definitions --------------------------------------------------------

#[derive(Debug)]
pub enum HtmlAttribute {
    Href,
}

#[derive(Debug)]
pub enum HtmlError {
    Internal { source: io::Error },
    InvalidAttribute,
    InvalidTag,
    NotFound,
    Utf8 { source: Utf8Error },
}

#[derive(Debug)]
pub enum HtmlTag {
    A,
}

pub struct Html {
    html: Arc<Mutex<Bytes>>,
}

// Html Impl ----------------------------------------------------------

impl Html {
    pub fn new(html: String) -> Self {
        Self {
            html: Arc::new(Mutex::new(Bytes::from(html))),
        }
    }
}

impl HtmlDocument for Html {
    fn bytes(&self) -> Bytes {
        self.html.clone().lock().unwrap().to_vec().into()
    }

    fn document(&self) -> Result<Document, HtmlError> {
        match Document::from_read(&**self.html.clone().lock().unwrap()) {
            Ok(doc) => Ok(doc),
            Err(err) => Err(HtmlError::from(err).into()),
        }
    }

    fn text(&self) -> Result<String, HtmlError> {
        match str::from_utf8(&self.html.clone().lock().unwrap()) {
            Ok(text) => Ok(text.to_string()),
            Err(err) => Err(HtmlError::from(err).into()),
        }
    }
}

// std Impl -----------------------------------------------------------

impl Display for HtmlAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HtmlAttribute::Href => write!(f, "href"),
        }
    }
}

impl Display for HtmlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal { source } => write!(f, "{}", source),
            Self::InvalidAttribute => write!(f, "invalid attribute"),
            Self::InvalidTag => write!(f, "invalid tag"),
            Self::NotFound => write!(f, "not found"),
            Self::Utf8 { source } => write!(f, "{}", source),
        }
    }
}

impl Display for HtmlTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "a"),
        }
    }
}

impl error::Error for HtmlError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Internal { source } => Some(source),
            Self::Utf8 { source } => Some(source),
            _ => None,
        }
    }
}

impl From<io::Error> for HtmlError {
    fn from(source: io::Error) -> Self {
        Self::Internal { source }
    }
}

impl From<Utf8Error> for HtmlError {
    fn from(source: Utf8Error) -> Self {
        Self::Utf8 { source }
    }
}

impl From<HtmlAttribute> for &str {
    fn from(attr: HtmlAttribute) -> Self {
        match attr {
            HtmlAttribute::Href => "href",
        }
    }
}

impl From<HtmlTag> for &str {
    fn from(tag: HtmlTag) -> Self {
        match tag {
            HtmlTag::A => "a",
        }
    }
}

impl FromStr for HtmlAttribute {
    type Err = HtmlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_lowercase() {
            "href" => Ok(HtmlAttribute::Href),
            _ => Err(HtmlError::InvalidAttribute),
        }
    }
}

impl FromStr for HtmlTag {
    type Err = HtmlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_lowercase() {
            "a" => Ok(HtmlTag::A),
            _ => Err(HtmlError::InvalidTag),
        }
    }
}
