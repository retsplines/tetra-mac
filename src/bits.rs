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

/// Convert a string of 1s and 0s into a Bits instance
/// The string my also contain space characters which have no effect on the resulting instance.
pub fn from_bitstr(s: &str) -> Bits {

    // Pack into bytes
    let mut bytes = Vec::new();
    let mut current = 0u8;
    let mut count = 0;

    for c in s.chars() {

        // Skip whitespace
        if c == ' ' {
            continue;
        }

        current <<= 1;
        current |= match c {
            '0' => 0,
            '1' => 1,
            _ => panic!("invalid char in bit string"),
        };

        count += 1;

        if count % 8 == 0 {
            bytes.push(current);
            current = 0;
        }
    }

    if count % 8 > 0 {
        // Pad the last byte (on the left since Msb0)
        current <<= 8 - (count % 8);
        bytes.push(current);
    }

    let mut bv = BitVec::<u8, Msb0>::from_vec(bytes);
    bv.truncate(count); // drop padded bits
    bv.shrink_to_fit();

    bv
}

mod tests {

    use bitvec::prelude::*;
    use crate::bits::from_bitstr;

    #[test]
    fn from_bitstr_behaves_identically_to_bitvec_macro() {

        let using_macro = bitvec![0, 1, 0, 1, 0, 0];
        let using_from_bitstr = from_bitstr("010100");
        assert_eq!(using_macro, using_from_bitstr);

        // Exact byte
        let using_macro = bitvec![0, 1, 0, 1, 0, 0, 1, 1];
        let using_from_bitstr = from_bitstr("0101 0011");
        assert_eq!(using_macro, using_from_bitstr);

        // Multi-byte sequences
        let using_macro = bitvec![0, 1, 1, 0,  0, 0, 0, 0,  1, 0, 1, 0,  1, 0, 0, 1,  1, 1, 0, 1];
        let using_from_bitstr = from_bitstr("0110 0000 1010 1001 1101");
        assert_eq!(using_macro, using_from_bitstr);

    }

}