use std::borrow::Borrow;
use std::fmt::Debug;
use std::ops::Deref;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Clone, Debug)]
pub struct HtmlParseOpts {
    /// Filter based on parent's HTML tag
    #[structopt(long, short)]
    pub tags: Option<Vec<String>>,
    /// File paths to be parsed
    #[structopt(parse(from_os_str), long, short)]
    pub paths: Option<Vec<PathBuf>>,
    /// Urls to be parsed
    #[structopt(long, short)]
    pub urls: Option<Vec<String>>
}

#[derive(StructOpt, Clone, Debug)]
pub enum HtmlOpts {
    /// Parse **\<a/>** tags from HTML document
    Links(HtmlParseOpts),
}

#[derive(StructOpt, Clone, Debug)]
pub enum DocumentOpts {
    /// Parse an HTML document
    Html(HtmlOpts),
}

#[derive(StructOpt, Clone, Debug)]
pub enum CommandOpts {
    /// Parses a specified document type
    #[structopt(name="probe")]
    Parse(DocumentOpts),
}

#[derive(StructOpt, Clone, Debug)]
pub struct Cli {
    /// System command options
    #[structopt(subcommand)]
    cmd: Option<CommandOpts>,
}

impl Cli {
    pub fn init() -> Self {
        Cli::from_args()
    }

    pub fn command(&self) -> &DocumentOpts {
        let cmd_opts = self.deref().borrow();
        match cmd_opts {
            CommandOpts::Parse(opts) => opts
        }
    }
}

impl Deref for Cli {
    type Target = CommandOpts;

    fn deref(&self) -> &Self::Target {
        self.cmd.as_ref().unwrap()
    }
}
