# reusable-fmt 

[![RUST](https://img.shields.io/badge/made%20with-RUST-red.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![crates.io](https://img.shields.io/crates/v/reusable-fmt.svg?color=green&style=for-the-badge&logo=hack-the-box&logoColor=yellow)](https://crates.io/crates/reusable-fmt)
[![docs.rs](https://img.shields.io/docsrs/reusable-fmt/latest?style=for-the-badge&logo=read-the-docs&logoColor=white)](https://docs.rs/reusable-fmt/latest/reusable_fmt)

*Reusable format strings for [std::fmt](https://doc.rust-lang.org/std/fmt/) macros*

**Initial Release**

This crate provides compile-time defined format string support for `std::fmt` macros like `write!`, `print!`, `format!`, etc.

## Installation

`Cargo.toml`:
```toml
[dependencies]
reusable-fmt = { git = https://github.com/rupansh/reusable-fmt }
```

`src.rs`:
```rust
use reusable_fmt::*;
```

## Example Usage
```rust
use reusable_fmt::*;

// This defines your format strings
fmt_reuse! {
    TEST1 = "This is a test! {}";
    TEST2 = "You can pass multiple format args! {} {}";
    TEST3 = r#"Raw Strings work too!! {}"#;
    TEST4 = "Named args {arg}";
    TEST5 = "Positional args {1} {0}";
    TEST6 = "Mixed {} {2} {1} {arg}";
}

fn main() {
	prntln!(TEST1, "Hello World"); // This is a test! Hello World
	let test = fmt!(TEST6, "Hello", "Test", "World", arg="Named"); // Mixed Hello World Test Named
	prntln!("{}", "WOW This works too!");
}
```

## Why
- Makes format strings less redundant
- No runtime overhead! everything is compile time.
- Dependency-free (unless you count build-dependencies)

## Contribution
Feel free to request and implement features.
I am not that good with macros so code improvements are welcome too!

### Testing
Tests should be run on nightly

`cargo +nightly test`

### Documentation
Documentation should be compiled on nightly

`cargo +nightly doc`
