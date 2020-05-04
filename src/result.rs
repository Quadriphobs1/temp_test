use std::fmt::{Debug, Error, Formatter};
use std::process::Output;
use std::str;

pub struct CommandResult {
  pub output: Output,
}

impl CommandResult {
  pub fn is_success(&self) -> bool {
    self.output.status.success()
  }

  pub fn is_err(&self) -> bool {
    !self.output.status.success()
  }

  pub fn stdout(&self) -> &str {
    str::from_utf8(&self.output.stdout).unwrap()
  }

  pub fn stderr(&self) -> &str {
    str::from_utf8(&self.output.stderr).unwrap()
  }

  pub fn code(&self) -> i32 {
    self.output.status.code().unwrap()
  }

  #[allow(dead_code)]
  pub fn result(self) -> Result<Self, Self> {
    if self.is_success() {
      Ok(self)
    } else {
      Err(self)
    }
  }
}

impl Debug for CommandResult {
  fn fmt(&self, out: &mut Formatter) -> Result<(), Error> {
    write!(out, "stdout: {}\nstderr: {}", self.stdout(), self.stderr())
  }
}
