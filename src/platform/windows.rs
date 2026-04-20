use crate::Result;
use std::{fs::OpenOptions, path::Path};

#[cfg(windows)]
pub(crate) fn sync_dir(path: &Path) -> Result<()> {
    use std::os::windows::fs::OpenOptionsExt;

    const FILE_SHARE_READ: u32 = 0x00000001;
    const FILE_SHARE_WRITE: u32 = 0x00000002;
    const FILE_SHARE_DELETE: u32 = 0x00000004;
    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x02000000;

    let dir = OpenOptions::new()
        .read(true)
        .write(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)?;

    dir.sync_all()?;

    Ok(())
}
