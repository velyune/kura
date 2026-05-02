#![deny(unsafe_code)]
#![warn(
    unused,
    unused_crate_dependencies,
    unreachable_pub,
    missing_debug_implementations,
    deprecated_in_future
)]

mod binary;
mod bootstrap;
mod db;
mod error;
mod filename;
mod internal_key;
mod lock;
mod manifest;
mod options;
mod platform;
mod sequence;

#[cfg(test)]
mod test_utils;

pub use db::{Db, PrefixEntry};
pub use error::{Error, Result};
pub use options::{Options, SyncMode};
