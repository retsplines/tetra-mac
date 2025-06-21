use bitvec::prelude::*;
use crate::Bits;

/// Generates n1-n22 normal training sequence 1 (EN 300 392-2 9.4.4.3.2)
pub(crate) fn training_sequence_normal_1_bits() -> Bits {
    Bits::from_bitslice(bits![
        u8, Msb0;
        1, 1, 0, 1, 0, 0, 0, 0, 
        1, 1, 1, 0, 1, 0, 0, 1, 
        1, 1, 0, 1, 0, 0
    ])
}

/// Generates p1-p22 normal training sequence 2
pub(crate) fn training_sequence_normal_2_bits() -> Bits {
    Bits::from_bitslice(bits![
        u8, Msb0;
        0, 1, 1, 1, 1, 0, 1, 0, 
        0, 1, 0, 0, 0, 0, 1, 1,
        0, 1, 1, 1, 1, 0
    ])
}

/// Generates q1-q22 normal training sequence 3
pub(crate) fn training_sequence_normal_3_bits() -> Bits {
    Bits::from_bitslice(bits![
        u8, Msb0;
        1, 0, 1, 1, 0, 1, 1, 1,
        0, 0, 0, 0, 0, 1, 1, 0,
        1, 0, 1, 1, 0, 1
    ])
}

/// Generates the extended training sequence x1-x30 (EN 300 392-2 9.4.4.3.3)
pub(crate) fn training_sequence_extended_bits() -> Bits {
    Bits::from_bitslice(bits![
        u8, Msb0;
        1, 0, 0, 1, 1, 1, 0, 1,
        0, 0, 0, 0, 1, 1, 1, 0,
        1, 0, 0, 1, 1, 1, 0, 1,
        0, 0, 0, 0, 1, 1
    ])
}
/// Generates the synchronisation training sequence (EN 300 392-2 9.4.4.3.4)
pub(crate) fn training_sequence_sync_bits() -> Bits {
    Bits::from_bitslice(bits![
        u8, Msb0;
        1, 1, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 1, 1, 1, 0, 0,
        1, 1, 1, 0, 1, 0, 0, 1,
        1, 1, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 1, 1, 1
    ])
}
