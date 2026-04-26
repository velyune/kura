use kura::{Db, Error, Options};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::{Duration, Instant},
};
use tempfile::tempdir;

fn assert_is_file(path: &Path) {
    assert!(path.is_file(), "expected file at {}", path.display())
}

fn assert_is_dir(path: &Path) {
    assert!(path.is_dir(), "expected directory at {}", path.display())
}

fn wait_for_path(path: &Path, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;

    while Instant::now() < deadline {
        if path.exists() {
            return true;
        }
        thread::sleep(Duration::from_millis(10));
    }

    path.exists()
}

#[test]
fn open_bootstraps_database_layout() {
    const INITIAL_MANIFEST_FILENAME: &str = "MANIFEST-00000000000000000001";

    let temp = tempdir().expect("create temp dir");
    let db_path = temp.path().join("db");

    assert!(
        !db_path.exists(),
        "database path should not exist before open"
    );

    let db = Db::open(&db_path, Options::default()).expect("open new database");

    assert_eq!(db.path(), db_path.as_path());
    assert_is_dir(&db_path);
    assert_is_file(&db_path.join("LOCK"));
    assert_is_dir(&db_path.join("wal"));
    assert_is_dir(&db_path.join("sst"));
    assert_is_dir(&db_path.join("tmp"));

    let current = fs::read_to_string(db_path.join("CURRENT")).expect("read CURRENT");
    assert_eq!(current, format!("{INITIAL_MANIFEST_FILENAME}\n"));
    assert_is_file(&db_path.join(INITIAL_MANIFEST_FILENAME))
}

#[test]
fn open_rejects_invalid_layout() {
    let temp = tempdir().expect("create temp dir");
    let db_path = temp.path().join("db");

    fs::create_dir_all(&db_path).expect("create db root");
    fs::write(db_path.join("wal"), b"not a directory").expect("create \"wal\" file");

    let err = Db::open(&db_path, Options::default())
        .expect_err("expected open to fail for invalid layout");

    assert!(
        matches!(err, Error::InvalidLayout { .. }),
        "expected Error::InvalidLayout, got {:?}",
        err
    )
}

#[test]
fn open_returns_locked_while_database_is_locked_by_another_process() {
    let temp = tempdir().expect("create temp dir");
    let db_path = temp.path().join("db");
    let ready_path = temp.path().join("child-ready");
    let release_path = temp.path().join("child-release");

    let mut child = Command::new(env::current_exe().expect("resolve current test binary"))
        .arg("--ignored")
        .arg("--exact")
        .arg("child_process_holds_database_lock")
        .env("KURA_DB_PATH", &db_path)
        .env("KURA_CHILD_READY_PATH", &ready_path)
        .env("KURA_CHILD_RELEASE_PATH", &release_path)
        .spawn()
        .expect("spawn child lock holder");

    assert!(
        wait_for_path(&ready_path, Duration::from_secs(10)),
        "timed out waiting for child ready signal"
    );

    let err = Db::open(&db_path, Options::default())
        .expect_err("expected open to fail for locked database");

    assert!(
        matches!(err, Error::Locked),
        "expected Error::Locked, got {:?}",
        err
    );

    fs::write(&release_path, b"release").expect("write release signal");
    let status = child.wait().expect("wait for child process");
    assert!(status.success(), "child process should exit successfully")
}

#[test]
#[ignore]
fn child_process_holds_database_lock() {
    let db_path = PathBuf::from(env::var("KURA_DB_PATH").expect("missing db path"));
    let ready_path =
        PathBuf::from(env::var("KURA_CHILD_READY_PATH").expect("missing child ready path"));
    let release_path =
        PathBuf::from(env::var("KURA_CHILD_RELEASE_PATH").expect("missing child release path"));

    let _db = Db::open(&db_path, Options::default()).expect("child open database");
    fs::write(&ready_path, b"ready").expect("write ready signal");

    assert!(
        wait_for_path(&release_path, Duration::from_secs(10)),
        "timed out waiting for child release signal"
    )
}

#[test]
fn open_succeeds_after_drop_releases_lock() {
    let temp = tempdir().expect("create temp dir");
    let db_path = temp.path().join("db");

    let db = Db::open(&db_path, Options::default()).expect("open new database");
    drop(db);

    let reopened =
        Db::open(&db_path, Options::default()).expect("reopen after drop should succeed");

    assert_eq!(reopened.path(), db_path.as_path())
}
