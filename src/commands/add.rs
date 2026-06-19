use std::{fs, path::PathBuf, process::Command};

use anyhow::Result;

use crate::{extension::CommandExt, user::get_repo_dir};

pub fn add_file(file: PathBuf) -> Result<()> {
    let dir = get_repo_dir()?;
    let rel_path = file.strip_prefix("/")?;
    let dest = dir.join(rel_path);

    fs::create_dir_all(&dest)?;

    Command::new("cp").args([&file, &dest]).run()?;
    Command::new("git").arg("add").arg(rel_path).run()
}
