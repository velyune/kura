use crate::{
    Error, Result,
    bootstrap::{validate_dir, validate_optional_file},
    platform,
};
use fs2::FileExt;
use std::{
    fs::{File, OpenOptions},
    path::Path,
};

#[derive(Debug)]
pub(crate) struct DbLock {
    file: File,
}

impl DbLock {
    pub(crate) fn acquire(db_path: &Path) -> Result<Self> {
        validate_dir(db_path)?;

        let lock_path = db_path.join("LOCK");
        validate_optional_file(&lock_path)?;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&lock_path)?;

        match file.try_lock_exclusive() {
            Ok(()) => Ok(Self { file }),
            Err(err) if platform::is_lock_error(&err) => Err(Error::Locked),
            Err(err) => Err(Error::Io(err)),
        }
    }
}

impl Drop for DbLock {
    fn drop(&mut self) {
        let _ = self.file.unlock();
    }
}
