use std::{
    fs,
    path::{Path, PathBuf},
};
use tempfile::{TempDir, tempdir};

pub(crate) fn temp_db() -> (TempDir, PathBuf) {
    let temp = tempdir().expect("create temp dir");
    let db_path = temp.path().join("db");
    fs::create_dir_all(&db_path).expect("create db root");

    (temp, db_path)
}

pub(crate) fn create_file(path: &Path) {
    fs::write(path, b"").expect("create file")
}
