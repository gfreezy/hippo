use crate::gc::global_definition::BITS_PER_WORD;

// Constants
pub const AGE_BITS: usize = 4;
pub const LOCK_BITS: usize = 2;
pub const BIASED_LOCK_BITS: usize = 1;

pub const HASH_BITS: usize = 32;
pub const UNUSED_GAP_BITS: usize = 1;
pub const EPOCH_BITS: usize = 2;

// The biased locking code currently requires that the age bits be
// contiguous to the lock bits.
pub const LOCK_SHIFT: usize = 0;
pub const BIASED_LOCK_SHIFT: usize = LOCK_BITS;
pub const AGE_SHIFT: usize = LOCK_BITS + BIASED_LOCK_BITS;
pub const UNUSED_GAP_SHIFT: usize = AGE_SHIFT + AGE_BITS;
pub const HASH_SHIFT: usize = UNUSED_GAP_SHIFT + UNUSED_GAP_BITS;
pub const EPOCH_SHIFT: usize = HASH_SHIFT;

pub const LOCK_MASK: usize = right_n_bits(LOCK_BITS);
pub const LOCK_MASK_IN_PLACE: usize = LOCK_MASK << LOCK_SHIFT;
pub const BIASED_LOCK_MASK: usize = right_n_bits(LOCK_BITS + BIASED_LOCK_BITS);
pub const BIASED_LOCK_MASK_IN_PLACE: usize = BIASED_LOCK_MASK << LOCK_SHIFT;
pub const BIASED_LOCK_BIT_IN_PLACE: usize = 1 << BIASED_LOCK_SHIFT;
pub const AGE_MASK: usize = right_n_bits(AGE_BITS);
pub const AGE_MASK_IN_PLACE: usize = AGE_MASK << AGE_SHIFT;
pub const EPOCH_MASK: usize = right_n_bits(EPOCH_BITS);
pub const EPOCH_MASK_IN_PLACE: usize = EPOCH_MASK << EPOCH_SHIFT;

pub const HASH_MASK: usize = right_n_bits(HASH_BITS);
pub const HASH_MASK_IN_PLACE: usize = HASH_MASK << HASH_SHIFT;

pub const fn nth_bit(n: usize) -> usize {
    1 << n
}

pub const fn right_n_bits(n: usize) -> usize {
    nth_bit(n) - 1
}

pub const fn mask_bits(n: usize, m: usize) -> usize {
    n & m
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MarkWord(u64);

impl MarkWord {
    pub fn hash(self) -> i32 {
        mask_bits((self.0 >> HASH_SHIFT) as usize, HASH_MASK) as i32
    }
}

impl Default for MarkWord {
    fn default() -> Self {
        MarkWord(0)
    }
}
