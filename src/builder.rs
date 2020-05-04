use crate::project::*;
use std::collections::HashSet;
use std::io::prelude::*;
use std::{env, str};
use std::{fs, path::PathBuf};
use tempfile::Builder;

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
