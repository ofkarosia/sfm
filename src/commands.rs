use std::{ffi::OsString, path::PathBuf};
use clap::Subcommand;

pub mod add;
pub mod init;
pub mod clone;

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
    Init,
    Add {
        file: PathBuf,
    },
    Clone {
        url: String,
        #[command(subcommand)]
        rest: Option<Passthrough>
    },
    #[command(external_subcommand)]
    Passthrough(Vec<OsString>)
}
