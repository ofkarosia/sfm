use std::{ffi::OsString, path::PathBuf};
use clap::Subcommand;

pub mod add;
pub mod init;
pub mod clone;
pub mod cd;

#[derive(Debug, Subcommand)]
pub enum Passthrough {
    #[command(external_subcommand)]
    Args(Vec<OsString>)
}

pub fn process_resting_args(args: Option<Passthrough>) -> Option<Vec<OsString>> {
    args.map(|p| {
        let Passthrough::Args(args) = p;
        args
    })
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Init repo
    Init,
    /// Wrapper around `git add`
    Add {
        file: PathBuf,
    },
    /// Clone the repo and overwrite existing files
    Clone {
        url: String,
        #[command(subcommand)]
        rest: Option<Passthrough>
    },
    /// Jump to the repo directory in new shell
    Cd,
    #[command(external_subcommand)]
    Passthrough(Vec<OsString>)
}
