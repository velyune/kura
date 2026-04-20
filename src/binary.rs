use crate::{Error, Result};
use std::path::Path;

pub(crate) trait DecodeLe: Sized {
    const SIZE: usize;

    fn from_le_slice(bytes: &[u8]) -> Self;
}

impl DecodeLe for u16 {
    const SIZE: usize = size_of::<u16>();

    fn from_le_slice(bytes: &[u8]) -> Self {
        Self::from_le_bytes(bytes.try_into().expect("u16 slice has fixed size"))
    }
}

impl DecodeLe for u32 {
    const SIZE: usize = size_of::<u32>();

    fn from_le_slice(bytes: &[u8]) -> Self {
        Self::from_le_bytes(bytes.try_into().expect("u32 slice has fixed size"))
    }
}

impl DecodeLe for u64 {
    const SIZE: usize = size_of::<u64>();

    fn from_le_slice(bytes: &[u8]) -> Self {
        Self::from_le_bytes(bytes.try_into().expect("u64 slice has fixed size"))
    }
}

pub(crate) fn read_le<T: DecodeLe>(
    bytes: &[u8],
    offset: &mut usize,
    path: &Path,
    context: &str,
    field: &str,
) -> Result<T> {
    if bytes[*offset..].len() < T::SIZE {
        return Err(Error::Corruption {
            message: format!(
                "{context} is truncated while reading {field}: {}",
                path.display()
            ),
        });
    }

    let value = T::from_le_slice(&bytes[*offset..*offset + T::SIZE]);
    *offset += T::SIZE;

    Ok(value)
}

pub(crate) fn read_bytes(
    bytes: &[u8],
    offset: &mut usize,
    len: usize,
    path: &Path,
    context: &str,
    field: &str,
) -> Result<Vec<u8>> {
    if bytes[*offset..].len() < len {
        return Err(Error::Corruption {
            message: format!(
                "{context} is truncated while reading {field}: {}",
                path.display()
            ),
        });
    }

    let value = bytes[*offset..*offset + len].to_vec();
    *offset += len;

    Ok(value)
}
