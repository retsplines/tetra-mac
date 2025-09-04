use bitvec::prelude::*;
use crate::bits::Bits;
use crate::new_bits;

/// Generates the tail bits
fn tail_bits() -> Bits {
    new_bits![1, 1, 0, 0]
}