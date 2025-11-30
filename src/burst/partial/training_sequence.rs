use crate::bits::Bits;
use crate::bits::from_bitstr;

/// Generates n1-n22 normal training sequence 1 (EN 300 392-2 9.4.4.3.2)
pub(crate) fn training_sequence_normal_1_bits() -> Bits {
    from_bitstr("
        11010000
        11101001
        110100
    ")
}

/// Generates p1-p22 normal training sequence 2
pub(crate) fn training_sequence_normal_2_bits() -> Bits {
    from_bitstr("
        01111010
        01000011
        011110
    ")
}

/// Generates q1-q22 normal training sequence 3
pub(crate) fn training_sequence_normal_3_bits() -> Bits {
    from_bitstr("
        10110111
        00000110
        101101
    ")
}

/// Generates the extended training sequence x1-x30 (EN 300 392-2 9.4.4.3.3)
pub(crate) fn training_sequence_extended_bits() -> Bits {
    from_bitstr("
        10011101
        00001110
        10011101
        000011
    ")
}
/// Generates the synchronisation training sequence y1-38 (EN 300 392-2 9.4.4.3.4)
pub(crate) fn training_sequence_sync_bits() -> Bits {
    from_bitstr("
        11000001
        10011100
        11101001
        11000001
        100111
    ")
}
