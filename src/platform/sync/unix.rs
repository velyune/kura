use std::{fs::File, path::Path};

#[cfg_attr(coverage_nightly, coverage(off))]
pub(crate) fn sync_dir(path: &Path) -> crate::Result<()> {
    File::open(path)?.sync_all()?;

    Ok(())
}
