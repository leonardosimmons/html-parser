use html_parser::cli::{Cli, DocumentOpts};
use html_parser::file::File;
use html_parser::html::{Html, HtmlParser};
use html_parser::parser::Parser;

fn main() {
    let cli = Cli::init();

    match cli.command() {
        DocumentOpts::Html(opts) => match opts.paths.as_ref() {
            Some(paths) => paths
                .iter()
                .map(|path| {
                    path.clone()
                        .into_os_string()
                        .into_string()
                        .unwrap_or_default()
                })
                .for_each(|path| match File::new(&*path) {
                    Ok(file) => {
                        let parser = Parser::<Html>::new(file.into());
                        match parser.links() {
                            Ok(links) => links.iter().for_each(|link| println!("link: {}", link)),
                            Err(err) => println!("{}", err.to_string()),
                        }
                    }
                    Err(err) => println!("{}", err.to_string()),
                }),
            None => println!("no file paths were provided"),
        },
    }
}
