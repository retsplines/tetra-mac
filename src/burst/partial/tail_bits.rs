use bitvec::prelude::*;
use crate::Bits;

/// Generates the tail bits
fn tail_bits() -> Bits {
    Bits::from_bitslice(bits![
        u8, Msb0;
        1, 1, 0, 0
    ])
}