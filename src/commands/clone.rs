use std::{ffi::OsString, path::PathBuf, process::Command};
use anyhow::{Result, bail};

use crate::{extension::CommandExt, user::get_repo_dir};

fn overwrite_files(dir: PathBuf) -> Result<()> {
    for entry in dir.read_dir()?.filter_map(|e| match e {
        Ok(e) => Some(e),
        Err(_) => None,
    }) {
        let file_type = entry.file_type()?;
        let file_name = entry.file_name();

        if file_type.is_symlink() {
            bail!("Symlink is not supported")
        }

        let mut cmd = Command::new("sudo");
        let mut cmd = cmd.arg("cp").current_dir(&dir);

        if file_type.is_dir() {
            cmd = cmd.arg("-r")
        }

        cmd.arg(file_name).arg("/").run()?
    }
    
    Ok(())
}

pub fn clone_repo(url: String, rest: Option<Vec<OsString>>) -> Result<()> {
    let dir = get_repo_dir()?;
    let mut cmd = Command::new("git");
    let mut cmd = cmd.arg("clone").arg(url).arg(&dir);

    if let Some(args) = rest {
        cmd = cmd.args(args)
    }

    cmd.run()?;
    overwrite_files(dir)
}
