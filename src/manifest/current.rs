use crate::{Error, Result, filename, platform};
use std::{fs, fs::OpenOptions, io::Write, path::Path, str};

pub(super) fn parse(bytes: &[u8]) -> Result<String> {
    let text = str::from_utf8(bytes).map_err(|_| Error::Corruption {
        message: "CURRENT must be valid UTF-8 text".to_owned(),
    })?;

    let Some(manifest_filename) = text.strip_suffix('\n') else {
        return Err(Error::Corruption {
            message: "CURRENT must end with a single newline".to_owned(),
        });
    };

    if manifest_filename.is_empty() || manifest_filename.contains('\n') {
        return Err(Error::Corruption {
            message: "CURRENT must contain exactly one manifest filename line".to_owned(),
        });
    }

    filename::validate_manifest(manifest_filename)?;

    Ok(manifest_filename.to_owned())
}

pub(super) fn publish(db_path: &Path, manifest_filename: &str) -> Result<()> {
    filename::validate_manifest(manifest_filename)?;

    let tmp_path = db_path.join("CURRENT.tmp");
    let current_path = db_path.join("CURRENT");

    {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&tmp_path)?;

        file.write_all(manifest_filename.as_bytes())?;
        file.write_all(b"\n")?;
        file.sync_all()?;
    }

    fs::rename(&tmp_path, &current_path)?;
    platform::sync_dir(db_path)
}
