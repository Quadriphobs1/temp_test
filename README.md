# temp_test - temporary test utils for rust 🔨

[![Build Status](https://github.com/Quadriphobs1/temp_test/workflows/ci/badge.svg?branch=master&event=push)](https://github.com/Quadriphobs1/temp_test/actions)
[![Documentation](https://docs.rs/temp_test/badge.svg)](https://docs.rs/temp_test/)
[![crates.io](https://img.shields.io/crates/d/temp_test.svg)](https://crates.io/crates/temp_test)

A simple temporary file library for Rust. This library can create a temporary environment to test your rust cli library, run test against several commands and args even provide different environments. It uses [tempfile](https://github.com/Stebalien/tempfile) to securely handle all temporary files.

## Usage

Minimum required Rust version: 1.32.0

Add this to your `Cargo.toml`:

```toml
[dev-dependencies]
temp_test = "0.1.1"
```

## Example

```rust
use clap::{Arg, App};

fn main() {
    let matches = App::new("My Super Program")
        .arg(Arg::with_name("config")
        .arg(Arg::with_name("v")
            .short('v')
            .multiple(true)
            .about("Sets the level of verbosity"))
        .subcommand(App::new("test")
            .about("controls testing features")
            .arg(Arg::with_name("debug")
                .short('d')
                .about("print debug information verbosely")))
        .get_matches();

    // do some action with the matches..
}


#[test]
fn test_example() {
    let p = temp_test::build_project("test_example").build();

    assert!(p.has_file("Cargo.toml"), "Cargo.toml file does not exist");

    let result = p.command("test").run();

    assert!(result.is_success(), "Result was unsuccessful {:?}", result);
}
```

## License

Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE or http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
