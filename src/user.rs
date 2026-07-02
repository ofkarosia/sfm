use anyhow::Result;
use once_cell::sync::OnceCell;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::extension::CommandExt;

const REPO_REL_PATH: &str = ".sfm/repo";
static USER: OnceCell<String> = OnceCell::new();
static REPO_DIR: OnceCell<PathBuf> = OnceCell::new();

fn whoami() -> Result<String> {
    Command::new("whoami").output_checked()
}

pub fn get_user() -> Result<&'static str> {
    USER.get_or_try_init(whoami).map(|s| s.as_str())
}

pub fn is_root() -> Result<bool> {
    get_user().map(|s| s.trim() == "root")
}

fn gen_repo_dir() -> Result<PathBuf> {
    let mut path = PathBuf::from("/home");
    path.push(get_user()?.trim());
    path.push(REPO_REL_PATH);
    Ok(path)
}

pub fn get_repo_dir() -> Result<&'static Path> {
    REPO_DIR.get_or_try_init(gen_repo_dir).map(|d| d.as_path())
}
