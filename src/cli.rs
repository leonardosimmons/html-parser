use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::Deref;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct HtmlParseOpts {
    /// Parent of elements to be parsed
    #[structopt(long, short = "P", help = "Parent of elements to be parsed")]
    pub parent: Option<String>,
    /// File paths to be parsed
    #[structopt(parse(from_os_str), long, short, help = "File path locations of HTML documents")]
    pub paths: Option<Vec<PathBuf>>,
    /// HTML elements to be parsed based on CSS selector
    #[structopt(long, short, help = "HTML elements to be parsed based on CSS selector")]
    pub selector: Option<Vec<String>>,
    /// HTML tags to be parsed from document
    #[structopt(long, short, help = "HTML tags to be parsed from document")]
    pub tags: Option<Vec<String>>,
    /// Toggle rather or not to include tracing
    #[structopt(long, short = "T", help = "Toggles tracing")]
    pub trace: bool,
    /// Urls to be parsed
    #[structopt(long, short, help = "Urls to pull HTML from")]
    pub urls: Option<Vec<String>>,
}

#[derive(StructOpt, Debug)]
pub enum DocumentOpts {
    /// Parse an HTML document
    Html(HtmlParseOpts),
}

#[derive(StructOpt, Debug)]
pub struct Cli {
    /// System command options
    #[structopt(subcommand)]
    cmd: Option<DocumentOpts>,
}

impl Cli {
    /// Initializes CLI
    pub fn init() -> Self {
        Cli::from_args()
    }

    /// Returns command issued by CLI
    pub fn command(&self) -> &DocumentOpts {
        self.deref().borrow()
    }
}

impl Deref for Cli {
    type Target = DocumentOpts;

    fn deref(&self) -> &Self::Target {
        self.cmd.as_ref().unwrap()
    }
}
