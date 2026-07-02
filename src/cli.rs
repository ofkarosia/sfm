use anyhow::{Result, bail};
use clap::Parser;
use log::debug;

use crate::{
    commands::{
        Command, add::add_file, cd::cd_repo, clone::clone_repo, init::init_repo,
        process_resting_args, sync::sync_to_repo,
    },
    git::passthrough,
    user::is_root,
};

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Dead simple system file manager. \nRest arguments are passed to git directly."
)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

pub fn run() -> Result<()> {
    env_logger::init();

    if is_root()? {
        bail!("You should not run this program as root")
    }

    let args = Args::parse();
    debug!("{args:?}");

    match args.command {
        Command::Init => init_repo(),
        Command::Add { file } => add_file(file),
        Command::Clone { url, rest } => clone_repo(url, process_resting_args(rest)),
        Command::Cd => cd_repo(),
        Command::Sync { add } => sync_to_repo(add),
        Command::Passthrough(args) => passthrough(args),
    }
}
