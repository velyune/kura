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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_le_reads_integers_and_advances_offset() {
        let bytes = [
            0x01, 0x00, // u16 = 1
            0x02, 0x00, 0x00, 0x00, // u32 = 2
            0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // u64 = 3
        ];
        let mut offset = 0;

        let value_u16: u16 =
            read_le(&bytes, &mut offset, Path::new("test"), "test", "u16").expect("read u16");
        assert_eq!(value_u16, 1);
        assert_eq!(offset, 2);

        let value_u32: u32 =
            read_le(&bytes, &mut offset, Path::new("test"), "test", "u32").expect("read u32");
        assert_eq!(value_u32, 2);
        assert_eq!(offset, 6);

        let value_u64: u64 =
            read_le(&bytes, &mut offset, Path::new("test"), "test", "u64").expect("read u64");
        assert_eq!(value_u64, 3);
        assert_eq!(offset, 14);
    }

    #[test]
    fn read_le_returns_corruption_when_truncated() {
        let bytes: &[u8] = &[0x01];
        let mut offset = 0;

        let err = read_le::<u16>(bytes, &mut offset, Path::new("test"), "context", "field")
            .expect_err("read truncated value should return corruption");

        assert!(matches!(err, Error::Corruption { message }
            if message == "context is truncated while reading field: test"));
        assert_eq!(offset, 0);
    }

    #[test]
    fn read_bytes_reads_bytes_and_advances_offset() {
        let bytes = b"abcd";
        let mut offset = 1;

        let value = read_bytes(bytes, &mut offset, 3, Path::new("test"), "context", "field")
            .expect("read bytes");

        assert_eq!(value, b"bcd");
        assert_eq!(offset, 4)
    }

    #[test]
    fn read_bytes_returns_corruption_when_truncated() {
        let bytes = b"abcd";
        let mut offset = 0;

        let err = read_bytes(bytes, &mut offset, 5, Path::new("test"), "context", "field")
            .expect_err("read truncated bytes should return corruption");

        assert!(matches!(err, Error::Corruption {message}
            if message == "context is truncated while reading field: test"));
        assert_eq!(offset, 0)
    }
}
