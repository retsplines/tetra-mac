// Defines a few types that are used throughout for bit storage and manipulation
use bitvec::prelude::*;

// Bit field where the MSB is stored first (0th)
pub type Bits = BitVec<u8, Msb0>;

// Convenient macro that wraps bitvec! to create new Bits instances
#[macro_export] macro_rules! new_bits {
    ($($elem:expr),* $(,)?) => {
        bitvec::bitvec![u8, Msb0; $($elem),*]
    };
}

// Everywhere in tetra-mac, I'll be using u8 as the representation for a bit
// The rationale for this is that TETRA frames are suitably short that even in embedded environments
// it's unlikely that it will pose a significant issue memory-wise.

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Bit(u8);

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        Bit(if value { 0xff } else { 0x00 })
    }
}

impl Bit {

}
