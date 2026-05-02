use std::io::{Error, ErrorKind};

pub(crate) fn is_lock_error(err: &Error) -> bool {
    const ERROR_LOCK_VIOLATION: i32 = 33;

    err.kind() == ErrorKind::WouldBlock || matches!(err.raw_os_error(), Some(ERROR_LOCK_VIOLATION))
}
