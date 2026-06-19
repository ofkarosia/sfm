use std::path::PathBuf;

use argh::FromArgs;

pub mod add;
pub mod init;
pub mod clone;

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "init")]
/// Init repo
pub struct InitCommand {}

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "add")]
/// Add file
pub struct AddCommand {
    #[argh(positional)]
    pub file: PathBuf,
}

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "clone")]
/// Clone repo
pub struct CloneCommand {
    #[argh(positional)]
    pub url: String,
    #[argh(positional, greedy)]
    pub rest: Vec<String>
}

#[derive(Debug, FromArgs)]
#[argh(subcommand)]
pub enum Command {
    Init(InitCommand),
    Add(AddCommand),
    Clone(CloneCommand)
}
