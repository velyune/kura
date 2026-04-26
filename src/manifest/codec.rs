//! The on-disk layout is documented in `docs/storage_format.md`.

use super::state::{ManifestState, SstableDescriptor};
use crate::{
    Error, Result,
    binary::{read_bytes, read_le},
    sequence::SequenceNumber,
};
use std::{
    fs,
    fs::{File, OpenOptions},
    io::{ErrorKind, Write},
    path::Path,
};

const MANIFEST_MAGIC: &[u8] = b"KURAMNF";
const MANIFEST_FORMAT_VERSION: u16 = 1;

pub(super) fn load(path: &Path) -> Result<ManifestState> {
    let bytes = fs::read(path).map_err(|err| match err.kind() {
        ErrorKind::NotFound => Error::Corruption {
            message: format!("manifest not found: {}", path.display()),
        },
        _ => Error::Io(err),
    })?;

    decode_file(&bytes, path)
}

pub(super) fn write(path: &Path, state: &ManifestState) -> Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;

    file.write_all(MANIFEST_MAGIC)?;
    file.write_all(&MANIFEST_FORMAT_VERSION.to_le_bytes())?;
    write_snapshot(&mut file, state)?;
    file.sync_all()?;

    Ok(())
}

fn decode_file(bytes: &[u8], path: &Path) -> Result<ManifestState> {
    let mut offset: usize = 0;
    decode_header(bytes, &mut offset, path)?;
    decode_snapshot(bytes, &mut offset, path)
}

fn decode_header(bytes: &[u8], offset: &mut usize, path: &Path) -> Result<()> {
    let magic = read_bytes(
        bytes,
        offset,
        MANIFEST_MAGIC.len(),
        path,
        "manifest header",
        "magic",
    )?;

    if magic != MANIFEST_MAGIC {
        return Err(Error::Corruption {
            message: format!("invalid manifest magic: {}", path.display()),
        });
    }

    let version: u16 = read_le(bytes, offset, path, "manifest header", "format version")?;

    if version != MANIFEST_FORMAT_VERSION {
        return Err(Error::Corruption {
            message: format!(
                "unsupported manifest format version {version} in: {}",
                path.display()
            ),
        });
    }

    Ok(())
}

fn decode_snapshot(bytes: &[u8], offset: &mut usize, path: &Path) -> Result<ManifestState> {
    let next_file_number: u64 =
        read_le(bytes, offset, path, "manifest snapshot", "next file number")?;

    let last_sequence: u64 = read_le(bytes, offset, path, "manifest snapshot", "last sequence")?;

    let wal_file_count: u32 = read_le(bytes, offset, path, "manifest snapshot", "WAL file count")?;
    let wal_files = decode_wal_files(bytes, offset, path, wal_file_count)?;

    let sstable_count: u32 = read_le(bytes, offset, path, "manifest snapshot", "SSTable count")?;
    let sstables = decode_sstables(bytes, offset, path, sstable_count)?;

    Ok(ManifestState::new(
        next_file_number,
        SequenceNumber::new(last_sequence),
        wal_files,
        sstables,
    ))
}

fn decode_wal_files(
    bytes: &[u8],
    offset: &mut usize,
    path: &Path,
    wal_file_count: u32,
) -> Result<Vec<u64>> {
    let mut wal_files = Vec::with_capacity(wal_file_count as usize);

    for _ in 0..wal_file_count {
        wal_files.push(read_le::<u64>(
            bytes,
            offset,
            path,
            "manifest WAL files",
            "file number",
        )?);
    }

    Ok(wal_files)
}

fn decode_sstables(
    bytes: &[u8],
    offset: &mut usize,
    path: &Path,
    sstable_count: u32,
) -> Result<Vec<SstableDescriptor>> {
    let mut sstables = Vec::with_capacity(sstable_count as usize);

    for _ in 0..sstable_count {
        sstables.push(decode_sstable_descriptor(bytes, offset, path)?);
    }

    Ok(sstables)
}

fn decode_sstable_descriptor(
    bytes: &[u8],
    offset: &mut usize,
    path: &Path,
) -> Result<SstableDescriptor> {
    let file_number: u64 = read_le(
        bytes,
        offset,
        path,
        "manifest SSTable descriptor",
        "file number",
    )?;

    let min_user_key_len: u32 = read_le(
        bytes,
        offset,
        path,
        "manifest SSTable descriptor",
        "min user key length",
    )?;
    let min_user_key = read_bytes(
        bytes,
        offset,
        min_user_key_len as usize,
        path,
        "manifest SSTable descriptor",
        "min user key",
    )?;

    let max_user_key_len: u32 = read_le(
        bytes,
        offset,
        path,
        "manifest SSTable descriptor",
        "max user key length",
    )?;
    let max_user_key: Vec<u8> = read_bytes(
        bytes,
        offset,
        max_user_key_len as usize,
        path,
        "manifest SSTable descriptor",
        "max user key",
    )?;

    let min_sequence: u64 = read_le(
        bytes,
        offset,
        path,
        "manifest SSTable descriptor",
        "min sequence",
    )?;

    let max_sequence: u64 = read_le(
        bytes,
        offset,
        path,
        "manifest SSTable descriptor",
        "max sequence",
    )?;

    Ok(SstableDescriptor::new(
        file_number,
        min_user_key,
        max_user_key,
        SequenceNumber::new(min_sequence),
        SequenceNumber::new(max_sequence),
    ))
}

fn write_snapshot(file: &mut File, state: &ManifestState) -> Result<()> {
    file.write_all(&state.next_file_number().to_le_bytes())?;
    file.write_all(&state.last_sequence().get().to_le_bytes())?;

    let wal_count =
        u32::try_from(state.wal_files().len()).map_err(|_| Error::EncodingLimitExceeded {
            message: "manifest snapshot WAL file count exceeds u32".to_owned(),
        })?;
    file.write_all(&wal_count.to_le_bytes())?;

    for wal_file_number in state.wal_files() {
        file.write_all(&wal_file_number.to_le_bytes())?;
    }

    let sstable_count =
        u32::try_from(state.sstables().len()).map_err(|_| Error::EncodingLimitExceeded {
            message: "manifest snapshot SSTable count exceeds u32".to_owned(),
        })?;
    file.write_all(&sstable_count.to_le_bytes())?;

    for sstable in state.sstables() {
        write_sstable_descriptor(file, sstable)?;
    }

    Ok(())
}

fn write_sstable_descriptor(file: &mut File, sstable: &SstableDescriptor) -> Result<()> {
    file.write_all(&sstable.file_number().to_le_bytes())?;

    let min_user_key_len =
        u32::try_from(sstable.min_user_key().len()).map_err(|_| Error::EncodingLimitExceeded {
            message: "manifest SSTable descriptor min user key length exceeds u32".to_owned(),
        })?;
    file.write_all(&min_user_key_len.to_le_bytes())?;
    file.write_all(sstable.min_user_key())?;

    let max_user_key_len =
        u32::try_from(sstable.max_user_key().len()).map_err(|_| Error::EncodingLimitExceeded {
            message: "manifest SSTable descriptor max user key length exceeds u32".to_owned(),
        })?;
    file.write_all(&max_user_key_len.to_le_bytes())?;
    file.write_all(sstable.max_user_key())?;

    file.write_all(&sstable.min_sequence().get().to_le_bytes())?;
    file.write_all(&sstable.max_sequence().get().to_le_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        filename, manifest,
        test_utils::{create_file, temp_db},
    };

    #[test]
    fn write_then_load_roundtrips_manifest_state() {
        let (_temp, db_path) = temp_db();
        let manifest_path = db_path.join(filename::manifest(manifest::INITIAL_FILE_NUMBER));
        let state = ManifestState::new(
            7,
            SequenceNumber::new(1000),
            vec![3, 4],
            vec![
                SstableDescriptor::new(
                    5,
                    b"user:000".to_vec(),
                    b"user:0499".to_vec(),
                    SequenceNumber::new(1),
                    SequenceNumber::new(500),
                ),
                SstableDescriptor::new(
                    6,
                    b"user:0500".to_vec(),
                    b"user:0999".to_vec(),
                    SequenceNumber::new(501),
                    SequenceNumber::new(1000),
                ),
            ],
        );

        write(&manifest_path, &state).expect("write manifest state");
        let loaded_state = load(&manifest_path).expect("load manifest state");

        assert_eq!(loaded_state, state)
    }

    #[test]
    fn load_rejects_missing_manifest_file() {
        let (_temp, db_path) = temp_db();
        let manifest_path = db_path.join(filename::manifest(manifest::INITIAL_FILE_NUMBER));

        let err = load(&manifest_path).expect_err("load should reject missing manifest file");

        assert!(matches!(err, Error::Corruption {message}
                if message == format!("manifest not found: {}", manifest_path.display())))
    }

    #[test]
    fn load_rejects_truncated_manifest_header() {
        let (_temp, db_path) = temp_db();
        let manifest_path = db_path.join(filename::manifest(manifest::INITIAL_FILE_NUMBER));

        create_file(&manifest_path);
        let err = load(&manifest_path).expect_err("load should reject truncated manifest header");

        assert!(matches!(err, Error::Corruption {message}
            if message == format!("manifest header is truncated while reading magic: {}", manifest_path.display())))
    }

    #[test]
    fn load_rejects_invalid_manifest_magic() {
        let (_temp, db_path) = temp_db();
        let manifest_path = db_path.join(filename::manifest(manifest::INITIAL_FILE_NUMBER));

        fs::write(
            &manifest_path,
            [
                b"INVALID".as_slice(),
                &MANIFEST_FORMAT_VERSION.to_le_bytes(),
            ]
            .concat(),
        )
        .expect("write manifest with invalid magic");
        let err = load(&manifest_path).expect_err("load should reject invalid manifest magic");

        assert!(matches!(err, Error::Corruption {message}
            if message == format!("invalid manifest magic: {}", manifest_path.display())))
    }

    #[test]
    fn load_rejects_unsupported_manifest_format_version() {
        let (_temp, db_path) = temp_db();
        let manifest_path = db_path.join(filename::manifest(manifest::INITIAL_FILE_NUMBER));

        fs::write(
            &manifest_path,
            [MANIFEST_MAGIC, &(MANIFEST_FORMAT_VERSION + 1).to_le_bytes()].concat(),
        )
        .expect("write manifest with unsupported format version");
        let err = load(&manifest_path)
            .expect_err("load should reject unsupported manifest format version");

        assert!(matches!(err, Error::Corruption {message}
            if message == format!("unsupported manifest format version {} in: {}", MANIFEST_FORMAT_VERSION + 1, manifest_path.display())))
    }

    #[test]
    fn load_rejects_truncated_manifest_snapshot() {
        let (_temp, db_path) = temp_db();
        let manifest_path = db_path.join(filename::manifest(manifest::INITIAL_FILE_NUMBER));

        fs::write(
            &manifest_path,
            [MANIFEST_MAGIC, &MANIFEST_FORMAT_VERSION.to_le_bytes()].concat(),
        )
        .expect("write truncated manifest snapshot");
        let err = load(&manifest_path).expect_err("load should reject truncated manifest snapshot");

        assert!(matches!(err, Error::Corruption {message}
            if message == format!("manifest snapshot is truncated while reading next file number: {}", manifest_path.display())))
    }
}
