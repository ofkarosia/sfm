use std::{fs, path::PathBuf, process::Command, result};

use anyhow::Result;
use ignore::WalkBuilder;

use crate::{extension::CommandExt, user::get_repo_dir};

pub fn sync_to_repo(add: bool) -> Result<()> {
    let repo_dir = get_repo_dir()?;
    let walk = WalkBuilder::new(repo_dir)
        .add_custom_ignore_filename(".sfmignore")
        .build();

    let files = walk
        .filter_map(|e| match e {
            Ok(e) if e.file_type().unwrap().is_file() => Some(e),
            _ => None,
        })
        .map(|e| {
            let path = e.path().to_path_buf();
            e.path()
                .strip_prefix(repo_dir)
                .map(|p| (PathBuf::from("/").join(p), path))
        })
        .collect::<result::Result<Vec<_>, _>>()?;

    for (target, dest) in files {
        let tmeta = target.metadata()?;
        let dmeta = target.metadata()?;
        #[cfg(target_os = "linux")]
        let changed = tmeta.mtime() != dmeta.mtime() || tmeta.ctime() != dmeta.ctime();

        Command::new("cp").arg(&target).arg(&dest).current_dir(repo_dir).run()?
    }

    Ok(())
}
