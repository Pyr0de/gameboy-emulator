use std::path::PathBuf;

use clap::{CommandFactory, Parser, error::ErrorKind};

#[derive(Parser, Debug)]
struct ArgsCli {
    /// Rom file path
    pub file: Option<PathBuf>,

    #[arg(long)]
    pub debug: bool,
}

#[derive(Debug)]
pub struct Args {
    pub file: PathBuf,

    pub debug: bool,
}

impl Args {
    pub fn new() -> Option<Self> {
        let args = ArgsCli::parse();

        let file = args.file?;
        if !file.exists() {
            let mut cmd = ArgsCli::command();
            cmd.error(
                ErrorKind::ValueValidation,
                format!("file `{}` doesn't exist", file.to_str().unwrap()),
            )
            .exit();
        }

        Some(Args {
            file,
            debug: args.debug,
        })
    }
}

#[test]
fn verify_app() {
    ArgsCli::command().debug_assert();
}
