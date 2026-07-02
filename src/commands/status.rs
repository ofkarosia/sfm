use std::process::Command;
use anyhow::Result;

use crate::{commands::sync::sync_to_repo, extension::CommandExt, user::get_repo_dir};

pub fn check_repo_status() -> Result<()> {
    sync_to_repo(false)?;
    Command::new("git").arg("status").current_dir(get_repo_dir()?).run()
}
