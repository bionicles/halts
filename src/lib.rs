//! halts/src/lib.rs

// Prevent the use of unsafe Rust code.
#![forbid(unsafe_code)]
// Highlight unused code segments
#![warn(dead_code)]
// Warn about public interfaces that aren't covered by documentation
#![deny(missing_docs)]
// This will turn all warnings into errors, which can help enforce code quality but might be too strict for a development environment.
// #![deny(warnings)]
// You may also want to add configuration for documentation tests:
// Ensure that `cargo doc` builds without warnings
#![doc(test(attr(deny(warnings))))]
// Suggests idioms or best practices for Rust 2018
#![warn(rust_2018_idioms)]
// Warns when types that might be expected to implement Debug do not
#![warn(missing_debug_implementations)]
// Warns about possibly unnecessary extern crate declarations.
#![warn(unused_extern_crates)]
// Enforce more strict and idiomatic coding patterns
// #![warn(clippy::all)]
// annoying
// #![warn(clippy::pedantic)]
// Warn about missing examples in documentation
#![warn(clippy::missing_docs_in_private_items)]
// Lint against possibly confusing code, likely errors, etc.
#![warn(clippy::cargo)]
// Warn if the crate version has not been updated since the last git commit that touched src/
#![warn(clippy::cargo_common_metadata)]
// Warn about missing license or documentation fields in Cargo.toml
#![warn(clippy::multiple_crate_versions)]
// Warn about wildcard dependencies in Cargo.toml
#![warn(clippy::wildcard_dependencies)]
// Check for correct handling of licensing
#![warn(clippy::correctness)]
// Check for items that should be declared `pub`
#![deny(clippy::as_conversions)]
// Check for API guidelines on naming and best practices
// #![warn(clippy::nursery)]
#![allow(clippy::option_if_let_else)]
// Warn if there is missing error documentation
#![warn(clippy::missing_errors_doc)]
// Warn about use of deprecated items
#![warn(deprecated)]
// Warn about constructs that will become errors or change meaning in future Rust editions
#![warn(future_incompatible)]
// Ensure that all non-test functions are used
#![cfg_attr(test, warn(unused))]

pub mod halts;
pub use halts::*;
