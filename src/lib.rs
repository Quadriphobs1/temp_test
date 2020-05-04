//! Temporary test utils for rust 🔨
//!
//! A simple, efficient library for counting code in directories. This
//! functionality is also provided as a
//! [CLI utility](//github.com/XAMPPRocky/tokei). Tokei uses a small state
//! machine rather than regular expressions found in other code counters. Tokei
//! can accurately count a lot more edge cases such as nested comments, or
//! comment syntax inside string literals.
//!
//! # Examples
//!
//! Run a command against in your library.
//!
//! ```
//! use clap::{Arg, App};
//! fn main_test() {
//!   let matches = App::new("My Super Program")
//!     arg(Arg::with_name("config")
//!       .arg(Arg::with_name("v")
//!           .short('v')
//!           .multiple(true)
//!           .about("Sets the level of verbosity"))
//!       .subcommand(App::new("test")
//!           .about("controls testing features")
//!           .arg(Arg::with_name("debug")
//!               .short('d')
//!               .about("print debug information verbosely")))
//!       .get_matches();

//!   // do some action with the matches..
//! }
//!
//! #[test]
//! fn test_example() {
//!   let p = temp_test::build_project("test_example").build();

//!   assert!(p.has_file("Cargo.toml"), "Cargo.toml file does not exist");

//!   let result = p.command("test").arg("debug").run();

//!   assert!(result.is_success(), "Result was unsuccessful {:?}", result);
//! }
//!
//! ```
//!
//!

extern crate tempfile;

mod builder;

use self::builder::ProjectBuilder;

pub fn build_project(name: &str) -> ProjectBuilder {
  ProjectBuilder::new(name)
}
