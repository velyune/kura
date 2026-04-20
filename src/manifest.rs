mod codec;
mod current;
mod state;

use self::state::ManifestState;
use crate::{Error, Result, filename};
use std::{fs, path::Path};

pub(crate) const INITIAL_FILE_NUMBER: u64 = 1;

pub(crate) fn load_current(db_path: &Path) -> Result<ManifestState> {
    let current_path = db_path.join("CURRENT");
    let bytes = fs::read(current_path)?;
    let manifest_path = db_path.join(current::parse(&bytes)?);
    codec::load(&manifest_path)
}

pub(crate) fn recover_initial_current(db_path: &Path) -> Result<ManifestState> {
    let manifest_filename = filename::manifest(INITIAL_FILE_NUMBER);
    let manifest_path = db_path.join(&manifest_filename);

    let state = codec::load(&manifest_path)?;
    if !state.is_initial() {
        return Err(Error::Corruption {
            message: format!(
                "initial manifest contains non-initial state: {}",
                manifest_path.display()
            ),
        });
    }

    current::publish(db_path, &manifest_filename)?;

    Ok(state)
}

pub(crate) fn bootstrap(db_path: &Path) -> Result<ManifestState> {
    let state = ManifestState::initial();
    let manifest_filename = filename::manifest(INITIAL_FILE_NUMBER);
    let manifest_path = db_path.join(&manifest_filename);

    codec::write(&manifest_path, &state)?;
    current::publish(db_path, &manifest_filename)?;

    Ok(state)
}
