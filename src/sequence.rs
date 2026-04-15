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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::Arc, thread};

    const EXPECT: &str = "allocate sequence number";

    #[test]
    fn allocate_returns_monotonic_sequences() {
        let allocator = SequenceAllocator::new();

        assert_eq!(allocator.allocate().expect(EXPECT), SequenceNumber::new(1));
        assert_eq!(allocator.allocate().expect(EXPECT), SequenceNumber::new(2));
        assert_eq!(allocator.allocate().expect(EXPECT), SequenceNumber::new(3));
        assert_eq!(allocator.last_allocated(), SequenceNumber::new(3))
    }

    #[test]
    fn allocate_resumes_after_recovered_sequence() {
        let allocator = SequenceAllocator::from_last_allocated(SequenceNumber::new(2));

        assert_eq!(allocator.allocate().expect(EXPECT), SequenceNumber::new(3));
        assert_eq!(allocator.allocate().expect(EXPECT), SequenceNumber::new(4))
    }

    #[test]
    fn allocate_returns_sequence_overflow_at_u64_max() {
        let allocator = SequenceAllocator::from_last_allocated(SequenceNumber::new(u64::MAX));

        assert!(matches!(allocator.allocate(), Err(Error::SequenceOverflow)))
    }

    #[test]
    fn allocate_returns_unique_sequences_across_threads() {
        const THREADS: usize = 8;
        const ALLOCATIONS_PER_THREAD: usize = 64;

        let allocator = Arc::new(SequenceAllocator::new());

        let mut sequences: Vec<SequenceNumber> = (0..THREADS)
            .map(|_| {
                let allocator = Arc::clone(&allocator);
                thread::spawn(move || {
                    (0..ALLOCATIONS_PER_THREAD)
                        .map(|_| allocator.allocate().expect(EXPECT))
                        .collect::<Vec<_>>()
                })
            })
            .flat_map(|handle| handle.join().expect("join thread"))
            .collect();

        sequences.sort_unstable();

        let expected: Vec<SequenceNumber> = (1..=(THREADS * ALLOCATIONS_PER_THREAD) as u64)
            .map(SequenceNumber)
            .collect();

        assert_eq!(sequences, expected)
    }

    #[test]
    fn observe_recovered_does_not_move_sequence_backwards() {
        let allocator = SequenceAllocator::new();

        assert_eq!(allocator.allocate().expect(EXPECT), SequenceNumber::new(1));

        allocator.observe_recovered(SequenceNumber::new(10));
        allocator.observe_recovered(SequenceNumber::new(2));

        assert_eq!(allocator.last_allocated(), SequenceNumber::new(10));
        assert_eq!(allocator.allocate().expect(EXPECT), SequenceNumber::new(11))
    }
}
