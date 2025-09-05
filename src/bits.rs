// Defines a few types that are used throughout for bit storage and manipulation
use bitvec::prelude::*;

// Bit field where the MSB is stored first (0th)
pub type Bits = BitVec<usize, Msb0>;

// Convenient macro that wraps bitvec! to create new Bits instances
#[macro_export] macro_rules! new_bits {
    ($($elem:expr),* $(,)?) => {
        bitvec::bitvec![usize, Msb0; $($elem),*]
    };
}
