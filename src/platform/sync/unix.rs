use std::{fs::File, path::Path};

pub(crate) fn sync_dir(path: &Path) -> crate::Result<()> {
    File::open(path)?.sync_all()?;

    Ok(())
}
