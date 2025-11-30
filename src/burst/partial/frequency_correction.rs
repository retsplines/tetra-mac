use crate::{bits::Bits};
use crate::bits::from_bitstr;

/// Generates the frequency correction bits
pub fn frequency_correction_bits() -> Bits {
    from_bitstr("
        11111111
        00000000
        00000000
        00000000
        00000000
        00000000
        00000000
        00000000
        00000000
        11111111
    ")
}