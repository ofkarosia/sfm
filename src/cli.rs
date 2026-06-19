use anyhow::{Result, bail};
use clap::Parser;

use crate::{
    commands::{Command, add::add_file, clone::clone_repo, init::init_repo, process_resting_args},
    git::passthrough, user::is_root,
};

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

pub fn run() -> Result<()> {
    if is_root()? {
        bail!("You should not run this program as root")
    }

    let args = Args::parse();
    println!("{args:?}");
    
    match args.command {
        Command::Init => init_repo(),
        Command::Add { file } => add_file(file),
        Command::Clone { url, rest } => clone_repo(url, process_resting_args(rest)),
        Command::Passthrough(args) => passthrough(args)
    }
}
