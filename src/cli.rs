use std::path::PathBuf;

use anyhow::{Result, bail};
use argh::FromArgs;

use crate::{
    commands::{add::add_file, init::init_repo},
    git::passthrough, user::is_root,
};

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "init")]
/// Init repo
struct InitCommand {}

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "add")]
/// Add file
struct AddCommand {
    #[argh(positional)]
    file: PathBuf,
}

#[derive(Debug, FromArgs)]
#[argh(subcommand)]
enum Command {
    Init(InitCommand),
    Add(AddCommand),
}

#[derive(Debug, FromArgs)]
/// Dead simple system file manager
struct Args {
    #[argh(subcommand)]
    command: Option<Command>,
    #[argh(positional, greedy)]
    rest: Vec<String>,
}

pub fn run() -> Result<()> {
    if is_root()? {
        bail!("You should not run this program as root")
    }

    let args: Args = argh::from_env();

    if args.command.is_none() {
        return passthrough(args.rest);
    }

    match args.command.unwrap() {
        Command::Init(_) => init_repo(),
        Command::Add(AddCommand { file }) => add_file(file),
    }
}
