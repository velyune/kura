#![allow(dead_code)]

use crate::sequence::SequenceNumber;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum ValueType {
    Value = 1,
    Tombstone = 2,
}

impl ValueType {
    pub(crate) const fn to_byte(self) -> u8 {
        self as u8
    }

    pub(crate) const fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            1 => Some(Self::Value),
            2 => Some(Self::Tombstone),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq)]
pub(crate) struct InternalKey {
    user_key: Vec<u8>,
    sequence_number: SequenceNumber,
    value_type: ValueType,
}

impl InternalKey {
    pub(crate) fn new(
        user_key: Vec<u8>,
        sequence_number: SequenceNumber,
        value_type: ValueType,
    ) -> Self {
        Self {
            user_key,
            sequence_number,
            value_type,
        }
    }

    pub(crate) fn user_key(&self) -> &[u8] {
        &self.user_key
    }

    pub(crate) fn sequence_number(&self) -> SequenceNumber {
        self.sequence_number
    }

    pub(crate) fn value_type(&self) -> ValueType {
        self.value_type
    }

    pub(crate) fn as_ref(&self) -> InternalKeyRef<'_> {
        InternalKeyRef {
            user_key: &self.user_key,
            sequence_number: self.sequence_number,
            value_type: self.value_type,
        }
    }
}

#[derive(Eq, PartialEq)]
pub(crate) struct InternalKeyRef<'a> {
    user_key: &'a [u8],
    sequence_number: SequenceNumber,
    value_type: ValueType,
}

impl<'a> InternalKeyRef<'a> {
    pub(crate) fn new(
        user_key: &'a [u8],
        sequence_number: SequenceNumber,
        value_type: ValueType,
    ) -> Self {
        Self {
            user_key,
            sequence_number,
            value_type,
        }
    }

    pub(crate) fn user_key(&self) -> &'a [u8] {
        self.user_key
    }

    pub(crate) fn sequence_number(&self) -> SequenceNumber {
        self.sequence_number
    }

    pub(crate) fn value_type(&self) -> ValueType {
        self.value_type
    }
}

/// Compares internal keys by:
/// 1. `user_key` in ascending lexicographic order
/// 2. `sequence_number` in descending order
/// 3. `value_type` in ascending order
fn compare(lhs: &InternalKeyRef<'_>, rhs: &InternalKeyRef<'_>) -> Ordering {
    lhs.user_key()
        .cmp(rhs.user_key())
        .then_with(|| rhs.sequence_number().cmp(&lhs.sequence_number()))
        .then_with(|| lhs.value_type().to_byte().cmp(&rhs.value_type().to_byte()))
}

impl Ord for InternalKey {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(&self.as_ref(), &other.as_ref())
    }
}

impl PartialOrd for InternalKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for InternalKeyRef<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self, other)
    }
}

impl PartialOrd for InternalKeyRef<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orders_user_keys_ascending() {
        let lhs_owned = InternalKey::new(b"a".to_vec(), SequenceNumber::new(1), ValueType::Value);
        let rhs_owned = InternalKey::new(b"b".to_vec(), SequenceNumber::new(1), ValueType::Value);

        assert_eq!(lhs_owned.cmp(&rhs_owned), Ordering::Less);
        assert_eq!(lhs_owned.cmp(&lhs_owned), Ordering::Equal);
        assert_eq!(rhs_owned.cmp(&lhs_owned), Ordering::Greater);

        let lhs_borrowed = InternalKeyRef::new(b"a", SequenceNumber::new(1), ValueType::Value);
        let rhs_borrowed = InternalKeyRef::new(b"b", SequenceNumber::new(1), ValueType::Value);

        assert_eq!(lhs_borrowed.cmp(&rhs_borrowed), Ordering::Less);
        assert_eq!(lhs_borrowed.cmp(&lhs_borrowed), Ordering::Equal);
        assert_eq!(rhs_borrowed.cmp(&lhs_borrowed), Ordering::Greater);
    }

    #[test]
    fn orders_sequence_numbers_descending() {
        let lhs_owned = InternalKey::new(b"a".to_vec(), SequenceNumber::new(1), ValueType::Value);
        let rhs_owned = InternalKey::new(b"a".to_vec(), SequenceNumber::new(2), ValueType::Value);

        assert_eq!(lhs_owned.cmp(&rhs_owned), Ordering::Greater);
        assert_eq!(lhs_owned.cmp(&lhs_owned), Ordering::Equal);
        assert_eq!(rhs_owned.cmp(&lhs_owned), Ordering::Less);

        let lhs_borrowed = InternalKeyRef::new(b"a", SequenceNumber::new(1), ValueType::Value);
        let rhs_borrowed = InternalKeyRef::new(b"a", SequenceNumber::new(2), ValueType::Value);

        assert_eq!(lhs_borrowed.cmp(&rhs_borrowed), Ordering::Greater);
        assert_eq!(lhs_borrowed.cmp(&lhs_borrowed), Ordering::Equal);
        assert_eq!(rhs_borrowed.cmp(&lhs_borrowed), Ordering::Less);
    }

    #[test]
    fn orders_value_types_ascending() {
        let lhs_owned = InternalKey::new(b"a".to_vec(), SequenceNumber::new(1), ValueType::Value);
        let rhs_owned =
            InternalKey::new(b"a".to_vec(), SequenceNumber::new(1), ValueType::Tombstone);

        assert_eq!(lhs_owned.cmp(&rhs_owned), Ordering::Less);
        assert_eq!(lhs_owned.cmp(&lhs_owned), Ordering::Equal);
        assert_eq!(rhs_owned.cmp(&lhs_owned), Ordering::Greater);

        let lhs_borrowed = InternalKeyRef::new(b"a", SequenceNumber::new(1), ValueType::Value);
        let rhs_borrowed = InternalKeyRef::new(b"a", SequenceNumber::new(1), ValueType::Tombstone);

        assert_eq!(lhs_borrowed.cmp(&rhs_borrowed), Ordering::Less);
        assert_eq!(lhs_borrowed.cmp(&lhs_borrowed), Ordering::Equal);
        assert_eq!(rhs_borrowed.cmp(&lhs_borrowed), Ordering::Greater);
    }

    #[test]
    fn newest_version_sorts_first_for_the_same_user_key() {
        let mut keys_owned = [
            InternalKey::new(b"a".to_vec(), SequenceNumber::new(1), ValueType::Value),
            InternalKey::new(b"a".to_vec(), SequenceNumber::new(3), ValueType::Value),
            InternalKey::new(b"a".to_vec(), SequenceNumber::new(2), ValueType::Value),
        ];

        keys_owned.sort();

        assert_eq!(keys_owned[0].sequence_number(), SequenceNumber::new(3));
        assert_eq!(keys_owned[1].sequence_number(), SequenceNumber::new(2));
        assert_eq!(keys_owned[2].sequence_number(), SequenceNumber::new(1));

        let mut keys_borrowed = [
            InternalKeyRef::new(b"a", SequenceNumber::new(1), ValueType::Value),
            InternalKeyRef::new(b"a", SequenceNumber::new(3), ValueType::Value),
            InternalKeyRef::new(b"a", SequenceNumber::new(2), ValueType::Value),
        ];

        keys_borrowed.sort();

        assert_eq!(keys_borrowed[0].sequence_number(), SequenceNumber::new(3));
        assert_eq!(keys_borrowed[1].sequence_number(), SequenceNumber::new(2));
        assert_eq!(keys_borrowed[2].sequence_number(), SequenceNumber::new(1));
    }

    #[test]
    fn value_type_byte_mapping_is_stable() {
        assert_eq!(ValueType::Value.to_byte(), 1);
        assert_eq!(ValueType::Tombstone.to_byte(), 2);

        assert_eq!(ValueType::from_byte(1), Some(ValueType::Value));
        assert_eq!(ValueType::from_byte(2), Some(ValueType::Tombstone));
        assert_eq!(ValueType::from_byte(0), None);
        assert_eq!(ValueType::from_byte(255), None);
    }
}
