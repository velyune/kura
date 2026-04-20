use crate::{Error, Result};

pub(crate) const FILE_NUMBER_WIDTH: usize = 20;

pub(crate) fn manifest(file_number: u64) -> String {
    format!("MANIFEST-{file_number:0width$}", width = FILE_NUMBER_WIDTH)
}

pub(crate) fn manifest_number(filename: &str) -> Option<u64> {
    let suffix = filename.strip_prefix("MANIFEST-")?;

    if suffix.len() != FILE_NUMBER_WIDTH || !suffix.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }

    suffix.parse::<u64>().ok()
}

pub(crate) fn validate_manifest(filename: &str) -> Result<()> {
    if manifest_number(filename).is_none() {
        return Err(Error::Corruption {
            message: format!("invalid manifest filename: {filename}"),
        });
    }

    Ok(())
}
