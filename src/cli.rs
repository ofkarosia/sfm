use anyhow::{Result, bail};
use argh::FromArgs;

use crate::{
    commands::{AddCommand, CloneCommand, Command, add::add_file, clone::clone_repo, init::init_repo},
    git::passthrough, user::is_root,
};

#[derive(Debug, FromArgs)]
#[argh(
    help_triggers("-h", "--help", "help"),
    description = "Dead simple system file manager.\nRest arguments are passed to `git`."
)]
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
        if args.rest.is_empty() {
            bail!("No arguments specified")
        }

        return passthrough(args.rest);
    }

    match args.command.unwrap() {
        Command::Init(_) => init_repo(),
        Command::Add(AddCommand { file }) => add_file(file),
        Command::Clone(CloneCommand { url, rest }) => clone_repo(url, rest)
    }
}
