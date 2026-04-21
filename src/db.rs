mod classify;

use self::classify::{DbState, classify};
use crate::{Options, Result, bootstrap, lock::DbLock, manifest, sequence::SequenceAllocator};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Db {
    path: PathBuf,
    options: Options,
    #[expect(dead_code)]
    sequence: SequenceAllocator,
    _lock: DbLock,
}

impl Db {
    pub fn open(path: impl AsRef<Path>, options: Options) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        bootstrap::ensure_dir(&path)?;

        let lock = DbLock::acquire(&path)?;

        let state = match classify(&path)? {
            DbState::New => {
                bootstrap::ensure_layout(&path)?;
                manifest::bootstrap(&path)?
            }
            DbState::IncompleteBootstrap => {
                bootstrap::validate_layout(&path)?;
                manifest::recover_initial_current(&path)?
            }
            DbState::Existing => {
                bootstrap::validate_layout(&path)?;
                manifest::load_current(&path)?
            }
        };

        Ok(Self {
            path,
            options,
            sequence: SequenceAllocator::from_last_allocated(state.last_sequence()),
            _lock: lock,
        })
    }

    pub fn put(&self, _key: &[u8], _value: &[u8]) -> Result<()> {
        todo!("put")
    }

    pub fn get(&self, _key: &[u8]) -> Result<Option<Vec<u8>>> {
        todo!("get")
    }

    pub fn delete(&self, _key: &[u8]) -> Result<()> {
        todo!("delete")
    }

    pub fn scan_prefix(&self, _prefix: &[u8]) -> Result<Vec<PrefixEntry>> {
        todo!("scan_prefix")
    }

    pub fn sync(&self) -> Result<()> {
        todo!("sync")
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn options(&self) -> &Options {
        &self.options
    }
}

pub struct PrefixEntry {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}
