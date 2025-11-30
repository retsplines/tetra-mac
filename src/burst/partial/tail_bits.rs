use bitvec::prelude::*;
use crate::bits::Bits;
use crate::bits::from_bitstr;

/// Generates the tail bits
fn tail_bits() -> Bits {
    from_bitstr("1, 1, 0, 0")
}