use std::error::{self, Error};
use std::fmt::{self, Display, Formatter};
use std::fs::File as SystemFile;
use std::io::{self, BufReader, Read};

// Definitions --------------------------------------------------------

#[derive(Debug)]
pub enum FileError {
    Internal { source: io::Error },
}

pub struct File {
    content: String,
}

// File Impl ----------------------------------------------------------

impl File {
    pub fn new(path: &str) -> Result<File, FileError> {
        match File::from(path, String::new()) {
            Ok(content) => Ok(File { content }),
            Err(err) => Err(err),
        }
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    fn from(path: &str, mut buf: String) -> Result<String, FileError> {
        match SystemFile::open(path) {
            Ok(f) => {
                let mut b = BufReader::new(f);
                match b.read_to_string(&mut buf) {
                    Ok(_) => Ok(buf),
                    Err(err) => Err(FileError::from(err)),
                }
            }
            Err(err) => Err(FileError::from(err)),
        }
    }

    pub fn get(self) -> String {
        self.content
    }
}

// std Impl -----------------------------------------------------------

impl Display for FileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Internal { source } => write!(f, "{}", source),
        }
    }
}

impl error::Error for FileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Internal { source } => Some(source),
        }
    }
}

impl From<io::Error> for FileError {
    fn from(source: io::Error) -> Self {
        Self::Internal { source }
    }
}
