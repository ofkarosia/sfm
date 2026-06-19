use anyhow::Result;
use std::process::Command;

use crate::{extension::CommandExt, user::get_repo_dir};

pub fn passthrough(args: Vec<String>) -> Result<()> {
    let dir = get_repo_dir()?;
    Command::new("git").args(args).current_dir(dir).run()
}
