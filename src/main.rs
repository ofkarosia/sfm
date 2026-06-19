use anyhow::Result;

use crate::cli::run;

mod cli;
mod commands;
mod extension;
mod git;
mod user;

fn main() -> Result<()> {
    run()
}
