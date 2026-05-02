use std::io::{Error, ErrorKind};

pub(crate) fn is_lock_error(err: &Error) -> bool {
    err.kind() == ErrorKind::WouldBlock
}
