use crate::Bits;

pub enum FillBitCapacity {
    Octets(usize),
    Bits(usize)
}

/// Add fill bits according to ETSI EN 300 392-2 § 23.4.2.2
///
/// Returns the number of fill bits added
pub fn add_fill_bits(bits: &mut Bits, capacity: FillBitCapacity) -> usize {

    // If a MAC length indication is supplied, this is a number of octets, otherwise, the
    // remaining capacity of a MAC block is specified in bits
    let mut bit_capacity = match capacity {
        FillBitCapacity::Octets(bytes) => bytes * 8,
        FillBitCapacity::Bits(bits) => bits
    };

    // Take a copy of the original bits length
    let original_length = bits.len();

    if original_length >= bit_capacity {
        return 0;
    }

    // Add a '1' bit which indicates fill bits start
    bits.push(true);
    bit_capacity -= 1;

    // Add '0' bits upto bit_capacity
    for _ in original_length..bit_capacity {
        bits.push(false);
    }

    // Return the number of fill bits added
    bits.len() - original_length
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn adds_fill_bits_upto_x_octets() {

        let mut bits = Bits::from_vec(vec![
            0xff, 0x00
        ]);

        let added = add_fill_bits(&mut bits, FillBitCapacity::Octets(4));

        assert_eq!(added, 16);
        assert_eq!(bits.len(), 32);

        // Check the fill bits are correct
        assert!(bits[16]);
        assert!(bits[17..32].iter().all(|b| !b));
    }

    #[test]
    fn adds_fill_bits_upto_x_bits() {

        let mut bits = Bits::from_vec(vec![
            0xff, 0x00, 0xaa
        ]);

        let added = add_fill_bits(&mut bits, FillBitCapacity::Bits(32));

        assert_eq!(added, 8);
        assert_eq!(bits.len(), 32);

        // Check the fill bits are correct
        assert!(bits[24]);
        assert!(bits[25..32].iter().all(|b| !b));
    }

}
