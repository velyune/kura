use crate::{Options, Result};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Db {
    path: PathBuf,
    options: Options,
}

impl Db {
    pub fn open(_path: impl AsRef<Path>, _options: Options) -> Result<Self> {
        todo!("open")
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
