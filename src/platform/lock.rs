#[cfg(any(unix, windows))]
mod native;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(any(unix, windows))]
pub(crate) use native::*;
#[cfg(unix)]
pub(crate) use unix::*;
#[cfg(windows)]
pub(crate) use windows::*;
