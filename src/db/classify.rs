use crate::{Error, Result, filename, manifest};
use std::{fs, io::ErrorKind, path::Path};

#[derive(Debug, Eq, PartialEq)]
pub(super) enum DbState {
    New,
    IncompleteBootstrap,
    Existing,
}

pub(super) fn classify(db_path: &Path) -> Result<DbState> {
    let current_path = db_path.join("CURRENT");

    match fs::read(current_path) {
        Ok(_) => Ok(DbState::Existing),
        Err(err) if err.kind() == ErrorKind::NotFound => {
            let scan = scan_manifests(db_path)?;

            match (scan.count, scan.has_initial) {
                (0, _) => Ok(DbState::New),
                (1, true) => Ok(DbState::IncompleteBootstrap),
                _ => Err(Error::Corruption {
                    message: format!("invalid manifest bootstrap state: {}", db_path.display()),
                }),
            }
        }
        Err(err) => Err(Error::Io(err)),
    }
}

struct ManifestScan {
    count: u8,
    has_initial: bool,
}

fn scan_manifests(path: &Path) -> Result<ManifestScan> {
    let mut count: u8 = 0;
    let mut has_initial = false;

    for entry in fs::read_dir(path)? {
        let filename = entry?.file_name();

        let Some(filename) = filename.to_str() else {
            continue;
        };

        let Some(manifest_number) = filename::manifest_number(filename) else {
            continue;
        };

        count = count.saturating_add(1);
        has_initial |= manifest_number == manifest::INITIAL_FILE_NUMBER;

        if count >= 2 {
            break;
        }
    }

    Ok(ManifestScan { count, has_initial })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_file, temp_db};

    #[test]
    fn classifies_new_when_current_and_manifests_are_missing() {
        let (_temp, db_path) = temp_db();

        let state = classify(&db_path).expect("classify db state");
        assert_eq!(state, DbState::New)
    }

    #[test]
    fn classifies_incomplete_bootstrap_when_only_initial_manifest_exists() {
        let (_temp, db_path) = temp_db();

        create_file(&db_path.join(filename::manifest(manifest::INITIAL_FILE_NUMBER)));

        let state = classify(&db_path).expect("classify db state");
        assert_eq!(state, DbState::IncompleteBootstrap)
    }

    #[test]
    fn classifies_existing_when_current_exists() {
        let (_temp, db_path) = temp_db();

        create_file(&db_path.join("CURRENT"));

        let state = classify(&db_path).expect("classify db state");
        assert_eq!(state, DbState::Existing)
    }

    #[test]
    fn rejects_non_initial_manifest_without_current() {
        let (_temp, db_path) = temp_db();

        create_file(&db_path.join(filename::manifest(manifest::INITIAL_FILE_NUMBER + 1)));

        let err = classify(&db_path).expect_err("classify should reject invalid db state");
        assert!(matches!(err, Error::Corruption { .. }))
    }

    #[test]
    fn rejects_multiple_manifests_without_current() {
        let (_temp, db_path) = temp_db();

        create_file(&db_path.join(filename::manifest(manifest::INITIAL_FILE_NUMBER)));
        create_file(&db_path.join(filename::manifest(manifest::INITIAL_FILE_NUMBER + 1)));

        let err = classify(&db_path).expect_err("classify should reject invalid db state");
        assert!(matches!(err, Error::Corruption { .. }))
    }
}
