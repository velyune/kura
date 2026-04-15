#![allow(dead_code)]

use crate::sequence::SequenceNumber;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum ValueType {
    Value = 1,
    Tombstone = 2,
}

impl ValueType {
    pub const fn to_byte(self) -> u8 {
        self as u8
    }

    pub const fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            1 => Some(Self::Value),
            2 => Some(Self::Tombstone),
            _ => None,
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct InternalKey {
    user_key: Vec<u8>,
    sequence_number: SequenceNumber,
    value_type: ValueType,
}

impl InternalKey {
    pub fn new(user_key: Vec<u8>, sequence_number: SequenceNumber, value_type: ValueType) -> Self {
        Self {
            user_key,
            sequence_number,
            value_type,
        }
    }

    pub fn user_key(&self) -> &[u8] {
        &self.user_key
    }

    pub fn sequence_number(&self) -> SequenceNumber {
        self.sequence_number
    }

    pub fn value_type(&self) -> ValueType {
        self.value_type
    }

    pub fn as_ref(&self) -> InternalKeyRef<'_> {
        InternalKeyRef {
            user_key: &self.user_key,
            sequence_number: self.sequence_number,
            value_type: self.value_type,
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct InternalKeyRef<'a> {
    user_key: &'a [u8],
    sequence_number: SequenceNumber,
    value_type: ValueType,
}

impl<'a> InternalKeyRef<'a> {
    pub fn new(user_key: &'a [u8], sequence_number: SequenceNumber, value_type: ValueType) -> Self {
        Self {
            user_key,
            sequence_number,
            value_type,
        }
    }

    pub fn user_key(&self) -> &'a [u8] {
        self.user_key
    }

    pub fn sequence_number(&self) -> SequenceNumber {
        self.sequence_number
    }

    pub fn value_type(&self) -> ValueType {
        self.value_type
    }
}

/*
impl<'a> From<&'a InternalKey> for InternalKeyRef<'a> {
    fn from(value: &'a InternalKey) -> Self {
        value.as_ref()
    }
}
*/

/// Compares internal keys by:
/// 1. `user_key` in ascending lexicographic order
/// 2. `sequence_number` in descending order
/// 3. `value_type` in ascending order
fn compare_internal_keys(lhs: &InternalKeyRef<'_>, rhs: &InternalKeyRef<'_>) -> Ordering {
    lhs.user_key()
        .cmp(rhs.user_key())
        .then_with(|| rhs.sequence_number().cmp(&lhs.sequence_number()))
        .then_with(|| lhs.value_type().to_byte().cmp(&rhs.value_type().to_byte()))
}

impl Ord for InternalKey {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_internal_keys(&self.as_ref(), &other.as_ref())
    }
}

impl PartialOrd for InternalKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for InternalKeyRef<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_internal_keys(self, other)
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
    fn value_type_byte_mapping_is_stable() {
        assert_eq!(ValueType::Value.to_byte(), 1);
        assert_eq!(ValueType::Tombstone.to_byte(), 2);

        assert_eq!(ValueType::from_byte(1), Some(ValueType::Value));
        assert_eq!(ValueType::from_byte(2), Some(ValueType::Tombstone));
        assert_eq!(ValueType::from_byte(0), None);
        assert_eq!(ValueType::from_byte(255), None)
    }

    #[test]
    fn orders_user_keys_ascending() {
        let lhs = InternalKey::new(b"a".to_vec(), SequenceNumber::new(1), ValueType::Value);
        let rhs = InternalKey::new(b"b".to_vec(), SequenceNumber::new(1), ValueType::Value);

        assert_eq!(lhs.cmp(&rhs), Ordering::Less);
        assert_eq!(lhs.cmp(&lhs), Ordering::Equal);
        assert_eq!(rhs.cmp(&lhs), Ordering::Greater)
    }

    #[test]
    fn orders_sequence_numbers_descending() {
        let lhs = InternalKey::new(b"a".to_vec(), SequenceNumber::new(1), ValueType::Value);
        let rhs = InternalKey::new(b"a".to_vec(), SequenceNumber::new(2), ValueType::Value);

        assert_eq!(lhs.cmp(&rhs), Ordering::Greater);
        assert_eq!(lhs.cmp(&lhs), Ordering::Equal);
        assert_eq!(rhs.cmp(&lhs), Ordering::Less)
    }

    #[test]
    fn orders_value_types_ascending() {
        let lhs = InternalKey::new(b"a".to_vec(), SequenceNumber::new(1), ValueType::Value);
        let rhs = InternalKey::new(b"a".to_vec(), SequenceNumber::new(1), ValueType::Tombstone);

        assert_eq!(lhs.cmp(&rhs), Ordering::Less);
        assert_eq!(lhs.cmp(&lhs), Ordering::Equal);
        assert_eq!(rhs.cmp(&lhs), Ordering::Greater)
    }

    #[test]
    fn newest_version_sorts_first_for_the_same_user_key() {
        let mut keys = [
            InternalKey::new(b"a".to_vec(), SequenceNumber::new(1), ValueType::Value),
            InternalKey::new(b"a".to_vec(), SequenceNumber::new(3), ValueType::Value),
            InternalKey::new(b"a".to_vec(), SequenceNumber::new(2), ValueType::Value),
        ];

        keys.sort();

        assert_eq!(keys[0].sequence_number(), SequenceNumber::new(3));
        assert_eq!(keys[1].sequence_number(), SequenceNumber::new(2));
        assert_eq!(keys[2].sequence_number(), SequenceNumber::new(1));
    }
}
