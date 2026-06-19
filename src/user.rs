use anyhow::Result;
use std::{path::PathBuf, process::Command};

use crate::extension::CommandExt;

pub const REPO_DIR: &str = "./.sfm/repo";

pub fn get_user() -> Result<String> {
    Command::new("whoami").output_checked()
}

pub fn is_root() -> Result<bool> {
    get_user().map(|s| s.trim() == "root")
}

pub fn get_repo_dir() -> Result<PathBuf> {
    let mut path = PathBuf::from("/home");
    path.push(get_user()?.trim());
    path.push(REPO_DIR);
    Ok(path)
}
