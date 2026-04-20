use crate::Result;
use std::{fs::File, path::Path};

#[cfg(unix)]
pub(crate) fn sync_dir(path: &Path) -> Result<()> {
    File::open(path)?.sync_all()?;

    Ok(())
}
