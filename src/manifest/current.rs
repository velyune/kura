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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{manifest, test_utils::temp_db};

    #[test]
    fn publish_then_parse_roundtrips_current() {
        let (_temp, db_path) = temp_db();
        let manifest_filename = filename::manifest(manifest::INITIAL_FILE_NUMBER);

        publish(&db_path, &manifest_filename).expect("publish CURRENT");
        let bytes = fs::read(db_path.join("CURRENT")).expect("read CURRENT");
        let parsed = parse(&bytes).expect("parse CURRENT");

        assert_eq!(parsed, manifest_filename)
    }

    #[test]
    fn parse_valid_current() {
        let manifest_filename = filename::manifest(manifest::INITIAL_FILE_NUMBER);
        let current = format!("{manifest_filename}\n");

        let parsed = parse(current.as_bytes()).expect("parse valid CURRENT");

        assert_eq!(parsed, manifest_filename)
    }

    #[test]
    fn parse_rejects_current_with_non_utf8_current() {
        let current = b"MANIFEST-\xFF\n";

        let err = parse(current).expect_err("parse should reject invalid CURRENT");

        assert!(matches!(err, Error::Corruption {message}
            if message == "CURRENT must be valid UTF-8 text"))
    }

    #[test]
    fn parse_rejects_current_without_trailing_newline() {
        let current = filename::manifest(manifest::INITIAL_FILE_NUMBER);

        let err = parse(current.as_bytes()).expect_err("parse should reject invalid CURRENT");

        assert!(matches!(err, Error::Corruption {message}
            if message == "CURRENT must end with a single newline"))
    }

    #[test]
    fn parse_rejects_current_without_single_manifest_filename_line() {
        let manifest_filename = filename::manifest(manifest::INITIAL_FILE_NUMBER);

        for current in [
            "\n".to_owned(),
            format!("{manifest_filename}\n\n"),
            format!("{manifest_filename}\n{manifest_filename}\n"),
        ] {
            let err = parse(current.as_bytes()).expect_err("parse should reject invalid CURRENT");

            assert!(matches!(err, Error::Corruption { message }
                if message == "CURRENT must contain exactly one manifest filename line"))
        }
    }

    #[test]
    fn publish_replaces_existing_current() {
        let (_temp, db_path) = temp_db();
        let old_manifest_filename = filename::manifest(manifest::INITIAL_FILE_NUMBER);
        let new_manifest_filename = filename::manifest(manifest::INITIAL_FILE_NUMBER);

        publish(&db_path, &old_manifest_filename).expect("publish initial CURRENT");
        publish(&db_path, &new_manifest_filename).expect("replace CURRENT");

        let current = fs::read_to_string(db_path.join("CURRENT")).expect("read CURRENT");

        assert_eq!(current, format!("{new_manifest_filename}\n"))
    }

    #[test]
    fn publish_removes_temporary_current_file() {
        let (_temp, db_path) = temp_db();
        let manifest_filename = filename::manifest(manifest::INITIAL_FILE_NUMBER);

        publish(&db_path, &manifest_filename).expect("publish CURRENT");

        assert!(
            !db_path.join("CURRENT.tmp").exists(),
            "CURRENT.tmp should not remain after publish"
        )
    }
}
