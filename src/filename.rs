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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_formats_file_number_with_zero_padding() {
        assert_eq!(manifest(1), "MANIFEST-00000000000000000001");
        assert_eq!(manifest(u64::MAX), "MANIFEST-18446744073709551615");
    }

    #[test]
    fn manifest_number_parses_valid_manifest_filename() {
        assert_eq!(manifest_number("MANIFEST-00000000000000000001"), Some(1));
        assert_eq!(
            manifest_number("MANIFEST-18446744073709551615"),
            Some(u64::MAX)
        );
    }

    #[test]
    fn manifest_number_rejects_invalid_prefix() {
        assert_eq!(manifest_number("00000000000000000001"), None);
        assert_eq!(manifest_number("CURRENT-00000000000000000001"), None);
    }

    #[test]
    fn manifest_number_rejects_wrong_width() {
        assert_eq!(manifest_number("MANIFEST-1"), None);
        assert_eq!(manifest_number("MANIFEST-000000000000000000001"), None);
    }

    #[test]
    fn manifest_number_rejects_non_digits() {
        assert_eq!(manifest_number("MANIFEST-0000000000000000000x"), None);
        assert_eq!(manifest_number("MANIFEST-0000000000000000000."), None);
    }

    #[test]
    fn manifest_number_rejects_u64_overflow() {
        assert_eq!(manifest_number("MANIFEST-18446744073709551616"), None);
    }

    #[test]
    fn validate_manifest_rejects_invalid_manifest_filename() {
        let err = validate_manifest("MANIFEST-1")
            .expect_err("validate manifest should reject invalid filename");

        assert!(matches!(err, Error::Corruption { message }
            if message == "invalid manifest filename: MANIFEST-1"));
    }
}
