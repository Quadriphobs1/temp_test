use crate::command::TestCommand;
use std::io::prelude::*;
use std::str;
use std::{fs, path::Path};
use tempfile::TempDir;

pub struct Project {
  pub dir: TempDir,
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
