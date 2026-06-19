use anyhow::{Result, anyhow};
use std::process::{Command, ExitStatus};

pub trait ExitStatusExt {
    fn checked(self) -> Result<()>;
}

impl ExitStatusExt for ExitStatus {
    fn checked(self) -> Result<()> {
        if self.success() {
            Ok(())
        } else {
            Err(anyhow!("Command failed with code: {:?}", self.code()))
        }
    }
}

pub trait CommandExt {
    fn run(&mut self) -> Result<()>;
    fn output_checked(&mut self) -> Result<String>;
}

impl CommandExt for Command {
    fn run(&mut self) -> Result<()> {
        self.status()?.checked()
    }

    fn output_checked(&mut self) -> Result<String> {
        let output = self.output()?;
        output.status.checked()?;
        Ok(String::from_utf8(output.stdout)?)
    }
}
