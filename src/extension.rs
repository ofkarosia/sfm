use anyhow::{Result, anyhow};
use log::debug;
use std::process::{Command, ExitStatus};

pub trait ExitStatusExt {
    fn checked(self) -> Result<()>;
}

impl ExitStatusExt for ExitStatus {
    fn checked(self) -> Result<()> {
        if self.success() {
            return Ok(())
        }

        Err(match self.code() {
            Some(code) => anyhow!("Process exited with code: {code}"),
            None => anyhow!("Process aborted by signal"),
        })
    }
}

pub trait CommandExt {
    fn run(&mut self) -> Result<()>;
    fn output_checked(&mut self) -> Result<String>;
}

impl CommandExt for Command {
    fn run(&mut self) -> Result<()> {
        debug!("Running: {self:?}");
        self.status()?.checked()
    }

    fn output_checked(&mut self) -> Result<String> {
        debug!("Running: {self:?}");
        let output = self.output()?;
        output.status.checked()?;
        Ok(String::from_utf8(output.stdout)?)
    }
}
