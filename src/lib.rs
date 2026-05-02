#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![deny(unsafe_code)]
#![warn(
    unused,
    unused_crate_dependencies,
    unreachable_pub,
    missing_debug_implementations,
    deprecated_in_future
)]

#[cfg(not(any(unix, windows)))]
compile_error!("Kura currently supports only Unix and Windows targets.");

mod binary;
mod bootstrap;
mod db;
mod error;
mod filename;
mod internal_key;
mod manifest;
mod options;
mod platform;
mod sequence;

#[cfg(test)]
mod test_utils;

pub use db::{Db, PrefixEntry};
pub use error::{Error, Result};
pub use options::{Options, SyncMode};
