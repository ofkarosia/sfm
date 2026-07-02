use std::{env, process::Command};
use anyhow::Result;

use crate::{extension::CommandExt, user::get_repo_dir};

pub fn cd_repo() -> Result<()> {
    let shell = env::var("SHELL")?;
    let dir = get_repo_dir()?;

    Command::new(shell).current_dir(dir).run()
}
