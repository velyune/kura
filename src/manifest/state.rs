use super::INITIAL_FILE_NUMBER;
use crate::sequence::SequenceNumber;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct ManifestState {
    next_file_number: u64,
    last_sequence: SequenceNumber,
    wal_files: Vec<u64>,
    sstables: Vec<SstableDescriptor>,
}

impl ManifestState {
    pub(crate) fn new(
        next_file_number: u64,
        last_sequence: SequenceNumber,
        wal_files: Vec<u64>,
        sstables: Vec<SstableDescriptor>,
    ) -> Self {
        Self {
            next_file_number,
            last_sequence,
            wal_files,
            sstables,
        }
    }

    pub(crate) fn initial() -> Self {
        Self::new(
            INITIAL_FILE_NUMBER + 1,
            SequenceNumber::ZERO,
            Vec::new(),
            Vec::new(),
        )
    }

    pub(crate) fn is_initial(&self) -> bool {
        self.next_file_number == INITIAL_FILE_NUMBER + 1
            && self.last_sequence == SequenceNumber::ZERO
            && self.wal_files.is_empty()
            && self.sstables.is_empty()
    }

    pub(crate) fn next_file_number(&self) -> u64 {
        self.next_file_number
    }

    pub(crate) fn last_sequence(&self) -> SequenceNumber {
        self.last_sequence
    }

    pub(crate) fn wal_files(&self) -> &[u64] {
        &self.wal_files
    }

    pub(crate) fn sstables(&self) -> &[SstableDescriptor] {
        &self.sstables
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct SstableDescriptor {
    file_number: u64,
    min_user_key: Vec<u8>,
    max_user_key: Vec<u8>,
    min_sequence: SequenceNumber,
    max_sequence: SequenceNumber,
}

impl SstableDescriptor {
    pub(crate) fn new(
        file_number: u64,
        min_user_key: Vec<u8>,
        max_user_key: Vec<u8>,
        min_sequence: SequenceNumber,
        max_sequence: SequenceNumber,
    ) -> Self {
        Self {
            file_number,
            min_user_key,
            max_user_key,
            min_sequence,
            max_sequence,
        }
    }

    pub(crate) fn file_number(&self) -> u64 {
        self.file_number
    }

    pub(crate) fn min_user_key(&self) -> &[u8] {
        &self.min_user_key
    }

    pub(crate) fn max_user_key(&self) -> &[u8] {
        &self.max_user_key
    }

    pub(crate) fn min_sequence(&self) -> SequenceNumber {
        self.min_sequence
    }

    pub(crate) fn max_sequence(&self) -> SequenceNumber {
        self.max_sequence
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_is_initial() {
        let state = ManifestState::initial();

        assert!(state.is_initial())
    }
}
