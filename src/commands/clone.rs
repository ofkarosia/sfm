use std::{ffi::OsString, path::Path, process::Command};
use anyhow::{Result, bail};
use ignore::WalkBuilder;

use crate::{extension::CommandExt, user::get_repo_dir};

fn overwrite_files(repo_dir: &Path) -> Result<()> {
    let walk = WalkBuilder::new(repo_dir).add_custom_ignore_filename(".sfmignore").max_depth(Some(1)).build();

    for entry in walk.filter_map(|e| match e {
        Ok(e) if e.depth() != 0 => Some(e),
        _ => None,
    }) {
        let file_type = entry.file_type().unwrap();
        let file_name = entry.file_name();

        if file_type.is_symlink() {
            bail!("Symlink is not supported")
        }

        let mut cmd = Command::new("sudo");
        let mut cmd = cmd.arg("cp").current_dir(&repo_dir);

        if file_type.is_dir() {
            cmd = cmd.arg("-r")
        }

        cmd.arg(file_name).arg("/").run()?
    }
    
    Ok(())
}

pub fn clone_repo(url: String, rest: Option<Vec<OsString>>) -> Result<()> {
    let repo_dir = get_repo_dir()?;
    let mut cmd = Command::new("git");
    let mut cmd = cmd.arg("clone").arg(url).arg(&repo_dir);

    if let Some(args) = rest {
        cmd = cmd.args(args)
    }

    cmd.run()?;
    overwrite_files(repo_dir)
}
