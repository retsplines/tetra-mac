use crate::Bits;

pub enum FillBitCapacity {
    Octets(usize),
    Bits(usize)
}

/// Add fill bits according to ETSI EN 300 392-2 § 23.4.2.2
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
    for _ in 0..bit_capacity {
        bits.push(false);
    }

    // Return the number of fill bits added
    bits.len() - original_length
}
