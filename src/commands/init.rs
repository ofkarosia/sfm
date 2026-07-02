use std::{fs, process::Command};
use anyhow::Result;

use crate::{extension::CommandExt, user::get_repo_dir};

pub fn init_repo() -> Result<()> {
    let dir = get_repo_dir()?;
    fs::create_dir_all(dir)?;
    Command::new("git")
        .args(["init", "."])
        .current_dir(dir)
        .run()
}
