use anyhow::Result;
use std::process::Command;

use crate::extension::CommandExt;

pub fn passthrough(args: Vec<String>) -> Result<()> {
    let sub_cmd = &args[0];
    Command::new(sub_cmd).args(&args[1..]).run()
}
