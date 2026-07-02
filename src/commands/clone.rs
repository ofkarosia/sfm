use anyhow::{Result, bail};
use ego_tree::Tree;
use ignore::WalkBuilder;
use std::{
    collections::HashMap,
    ffi::OsString,
    fmt::Display,
    io::stdin,
    path::Path,
    process::{self, Command},
};

use crate::{extension::CommandExt, user::get_repo_dir};

#[derive(Debug)]
struct PathNode {
    path: OsString,
    is_dir: bool,
}

impl PathNode {
    fn new(path: OsString, is_dir: bool) -> Self {
        Self { path, is_dir }
    }
}

impl Display for PathNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.path.display()))
    }
}

fn overwrite_files(repo_dir: &Path) -> Result<()> {
    let walk = WalkBuilder::new(repo_dir)
        .add_custom_ignore_filename(".sfmignore")
        .build();

    let mut tree = Tree::new(PathNode::new(OsString::from("/"), true));
    let mut id_map = HashMap::new();

    let root_id = tree.root().id();

    for entry in walk.filter_map(|e| match e {
        Ok(e) if e.depth() != 0 => Some(e),
        _ => None,
    }) {
        let file_type = entry.file_type().unwrap();
        let file_name = entry.file_name();

        if file_type.is_symlink() {
            bail!("Symlink is not supported")
        }

        let path = entry.path();

        let parent_id = if entry.depth() == 1 {
            root_id
        } else {
            *id_map.get(path.parent().unwrap().as_os_str()).unwrap()
        };

        let is_dir = file_type.is_dir();
        let id = tree
            .get_mut(parent_id)
            .unwrap()
            .append(PathNode::new(file_name.to_os_string(), is_dir))
            .id();

        if is_dir {
            id_map.insert(path.as_os_str().to_os_string(), id);
        }
    }

    println!("The following files will be overwrited:");
    println!("{tree}");
    println!("Proceed to overwrite? (y/n):");

    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    let proceed = match buf.as_str() {
        "y" | "Y" => true,
        "n" | "N" => false,
        _ => bail!("Unrecognized input"),
    };

    if !proceed {
        process::exit(1)
    }

    for node in tree.get(root_id).unwrap().children() {
        let node = node.value();
        let mut cmd = Command::new("sudo");
        let mut cmd = cmd.arg("cp").current_dir(repo_dir);

        if node.is_dir {
            cmd = cmd.arg("-r")
        }

        cmd.arg(&node.path).arg("/").run()?
    }

    Ok(())
}

pub fn clone_repo(url: String, rest: Option<Vec<OsString>>) -> Result<()> {
    let repo_dir = get_repo_dir()?;
    let mut cmd = Command::new("git");
    let mut cmd = cmd.arg("clone").arg(url).arg(repo_dir);

    if let Some(args) = rest {
        cmd = cmd.args(args)
    }

    cmd.run()?;
    overwrite_files(repo_dir)
}
