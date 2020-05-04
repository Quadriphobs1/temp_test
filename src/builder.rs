use std::collections::HashSet;
use std::fmt::{Debug, Error, Formatter};
use std::io::prelude::*;
use std::process::{Command, Output};
use std::{env, str};
use std::{
  fs,
  path::{Path, PathBuf},
};
use tempfile::{Builder, TempDir};

pub struct ProjectBuilder {
  name: String,
  folders: HashSet<String>,
  files: HashSet<(PathBuf, String)>,
  package_name: Option<String>,
}

impl ProjectBuilder {
  pub fn new(name: &str) -> Self {
    ProjectBuilder {
      name: name.into(),
      folders: HashSet::new(),
      files: HashSet::new(),
      package_name: None,
    }
  }

  pub fn package_name(mut self, name: &str) -> Self {
    self.package_name = Some(name.into());
    self
  }

  pub fn folder(mut self, name: &str) -> Self {
    self.folders.insert(name.into());
    self
  }

  pub fn file(mut self, name: &str, contents: &str) -> Self {
    self.files.insert((name.into(), contents.into()));
    self
  }

  pub fn build(self) -> Project {
    let dir = Builder::new().prefix(&self.name).tempdir().unwrap();
    let temp_path = dir.path().canonicalize().unwrap();
    let cargo_path = temp_path.join("Cargo.toml");

    fs::File::create(&cargo_path).unwrap();

    for folder in self.folders {
      fs::create_dir(temp_path.join(folder)).unwrap();
    }

    for (file, contents) in self.files {
      fs::File::create(temp_path.join(file))
        .unwrap()
        .write_all(contents.as_bytes())
        .unwrap()
    }

    if let Some(package_name) = self.package_name {
      env::set_var("PACKAGE_NAME", package_name);
    }

    Project {
      dir,
      name: self.name,
    }
  }
}

pub struct Project {
  dir: TempDir,
  pub name: String,
}

impl Project {
  pub fn command(&self, command: &str) -> TestCommand {
    TestCommand::new(self.dir.path(), command)
  }

  pub fn has_file<P: AsRef<Path>>(&self, path: P) -> bool {
    self.dir.path().join(path).exists()
  }

  pub fn file_contents<P: AsRef<Path>>(&self, path: P) -> String {
    let mut f = fs::File::open(self.dir.path().join(path)).expect("Could not open file");
    let mut result = String::new();
    f.read_to_string(&mut result).expect("Could not read file");
    result
  }

  pub fn delete_file<P: AsRef<Path>>(&self, path: P) {
    let file = self.dir.path().join(path);
    fs::remove_dir_all(file).unwrap();
  }
}

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

pub struct CommandResult {
  output: Output,
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
