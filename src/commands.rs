use clap::Subcommand;
use std::{ffi::OsString, path::PathBuf};

pub mod add;
pub mod cd;
pub mod clone;
pub mod init;
pub mod sync;
pub mod status;

#[derive(Debug, Subcommand)]
pub enum Passthrough {
    #[command(external_subcommand)]
    Args(Vec<OsString>),
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
    /// Wrapped version of `git add`
    Add {
        /// Path to the local file
        file: PathBuf,
    },
    /// Clone the repo and overwrite existing files
    Clone {
        /// URL of the repo
        url: String,
        #[command(subcommand)]
        rest: Option<Passthrough>,
    },
    /// Jump to the repo directory in new shell
    Cd,
    /// Synchronize local files to repo
    Sync {
        /// Wrapped version of `git add -u`
        #[arg(short = 'A', long)]
        add: bool,
    },
    /// Equivalent to `sfm sync && git status` in the repo directory
    Status,
    #[command(external_subcommand)]
    Passthrough(Vec<OsString>),
}
