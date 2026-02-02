use std::path::PathBuf;

use clap::{CommandFactory, Parser, error::ErrorKind};

#[derive(Parser, Debug)]
pub(crate) struct Args {
    /// Rom file path
    pub file: PathBuf,

    #[arg(long)]
    pub debug: bool,
}

impl Args {
    pub fn new() -> Self {
        let args = Self::parse();
        if !args.file.exists() {
            let mut cmd = Args::command();
            cmd.error(
                ErrorKind::ValueValidation,
                format!("file `{}` doesn't exist", args.file.to_str().unwrap()),
            )
            .exit();
        }

        args
    }
}

#[test]
fn verify_app() {
    Args::command().debug_assert();
}
