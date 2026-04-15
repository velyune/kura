#![allow(dead_code)]

use crate::{Error, Result};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SequenceNumber(u64);

impl SequenceNumber {
    pub const ZERO: Self = Self(0);

    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug)]
pub struct SequenceAllocator {
    last_allocated: AtomicU64,
}

impl SequenceAllocator {
    pub const fn new() -> Self {
        Self::from_last_allocated(SequenceNumber::ZERO)
    }

    pub const fn from_last_allocated(last_allocated: SequenceNumber) -> Self {
        Self {
            last_allocated: AtomicU64::new(last_allocated.get()),
        }
    }

    pub fn allocate(&self) -> Result<SequenceNumber> {
        let previous = self
            .last_allocated
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |current| {
                current.checked_add(1)
            })
            .map_err(|_| Error::SequenceOverflow)?;

        Ok(SequenceNumber::new(previous + 1))
    }

    pub fn last_allocated(&self) -> SequenceNumber {
        SequenceNumber::new(self.last_allocated.load(Ordering::SeqCst))
    }

    pub fn observe_recovered(&self, recovered: SequenceNumber) {
        self.last_allocated
            .fetch_max(recovered.get(), Ordering::SeqCst);
    }
}

impl Default for SequenceAllocator {
    fn default() -> Self {
        Self::new()
    }
}
