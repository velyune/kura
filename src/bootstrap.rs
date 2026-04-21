use crate::{Error, Result};
use std::{fs, io::ErrorKind, path::Path};

enum EntryType {
    File,
    Directory,
}

enum ExistenceMode {
    AllowMissing,
    MustExist,
}

fn validate_entry(path: &Path, entry_type: EntryType, mode: ExistenceMode) -> Result<()> {
    match fs::metadata(path) {
        Ok(meta) => match entry_type {
            EntryType::File if meta.is_file() => Ok(()),
            EntryType::Directory if meta.is_dir() => Ok(()),
            EntryType::File => Err(Error::InvalidLayout {
                message: format!("expected file, found different type: {}", path.display()),
            }),
            EntryType::Directory => Err(Error::InvalidLayout {
                message: format!(
                    "expected directory, found different type: {}",
                    path.display()
                ),
            }),
        },
        Err(err) if err.kind() == ErrorKind::NotFound => match mode {
            ExistenceMode::AllowMissing => Ok(()),
            ExistenceMode::MustExist => Err(Error::InvalidLayout {
                message: format!("path not found: {}", path.display()),
            }),
        },
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn validate_optional_file(path: &Path) -> Result<()> {
    validate_entry(path, EntryType::File, ExistenceMode::AllowMissing)
}

pub fn validate_dir(path: &Path) -> Result<()> {
    validate_entry(path, EntryType::Directory, ExistenceMode::MustExist)
}

pub fn ensure_dir(path: &Path) -> Result<()> {
    match fs::metadata(path) {
        Ok(_) => validate_dir(path),
        Err(err) if err.kind() == ErrorKind::NotFound => {
            fs::create_dir_all(path)?;
            validate_dir(path)
        }
        Err(err) => Err(Error::Io(err)),
    }
}

pub fn validate_layout(db_path: &Path) -> Result<()> {
    validate_dir(db_path)?;

    validate_dir(&db_path.join("wal"))?;
    validate_dir(&db_path.join("sst"))?;
    validate_dir(&db_path.join("tmp"))
}

pub fn ensure_layout(db_path: &Path) -> Result<()> {
    validate_dir(db_path)?;

    ensure_dir(&db_path.join("wal"))?;
    ensure_dir(&db_path.join("sst"))?;
    ensure_dir(&db_path.join("tmp"))
}
