use anyhow::Result;
use std::{ffi::OsString, process::Command};

use crate::{extension::CommandExt, user::get_repo_dir};

pub fn passthrough(args: Vec<OsString>) -> Result<()> {
    let dir = get_repo_dir()?;
    Command::new("git").args(args).current_dir(dir).run()
}
