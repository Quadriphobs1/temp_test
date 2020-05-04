use crate::result::CommandResult;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, str};

pub struct TestCommand {
  cwd: PathBuf,
  args: Vec<String>,
  env_vars: Vec<(String, String)>,
}

impl TestCommand {
  pub fn new(cwd: &Path, subcommand: &str) -> Self {
    TestCommand {
      cwd: cwd.into(),
      args: vec![subcommand.into()],
      env_vars: Vec::new(),
    }
  }

  pub fn arg<S: Into<String>>(mut self, value: S) -> Self {
    self.args.push(value.into());
    self
  }

  pub fn args<I>(self, values: I) -> Self
  where
    I: IntoIterator,
    I::Item: Into<String>,
  {
    values.into_iter().fold(self, |c, value| c.arg(value))
  }

  pub fn env(mut self, key: &str, value: &str) -> Self {
    self.env_vars.push((key.into(), value.into()));
    self
  }

  pub fn cd<P: AsRef<Path>>(mut self, path: P) -> Self {
    self.cwd.push(path);
    self
  }

  pub fn run(self) -> CommandResult {
    let output = self.build_command().output().unwrap();
    CommandResult { output }
  }

  fn build_command(&self) -> Command {
    let env_package_name = env::var("CARGO_PKG_NAME").unwrap();
    let project_path = Path::new(&env::var_os("CARGO_MANIFEST_DIR").unwrap())
      .parent()
      .unwrap()
      .join("target")
      .join("debug")
      .join(env::var("PACKAGE_NAME").unwrap_or(env_package_name));
    let mut command = Command::new(project_path);
    command.args(&self.args).current_dir(&self.cwd);
    for &(ref k, ref v) in self.env_vars.iter() {
      command.env(&k, &v);
    }
    command
  }
}
