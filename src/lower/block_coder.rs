use bitvec::prelude::{*};
use bitvec::view::BitView;
use log::info;
use crate::bits::Bits;

#[derive(Debug)]
pub struct BlockError {
    indicated: u16,
    calculated: u16
}

fn compute(block: &Bits) -> u16 {

    let mut crc = 0xffffu16;
    for bit in block {

        crc ^= (*bit as u16) << 15;

        // First bit set?
        if crc & 0x8000 > 0 {
            crc <<= 1;
            crc ^= 0x1021;
        } else {
            crc <<= 1;
        }
    }

    !crc
}

/// Implements the (K1+16, K1) block code specified in ETSI EN 300 392-2 §8.2.3.3
/// Effectively appends a CRC to the block.
pub fn block_encode(block: &Bits) -> Bits {

    let crc = compute(block);

    // Append the CRC bits to the end of the block
    let mut out = block.clone();
    out.extend_from_bitslice(&crc.view_bits::<Msb0>()[..16]);

    out
}

/// Decode the (K1+16, K1) block code specified in ETSI EN 300 392-2 §8.2.3.3
pub fn block_decode(block: &Bits) -> Result<Bits, BlockError> {

    // Block must be at least 17 bits to be able to be validated
    assert!(block.len() > 17, "require >= 17 bits for block-decode");

    // Strip the final 16 bits which contain the checksum
    let (decoded, crc_bits) = block.split_at(block.len() - 16);
    info!("{}", crc_bits);
    let indicated: u16 = crc_bits.load_be();

    // Copy the block bits without the checksum into a new vec
    let decoded: Bits = Bits::from_bitslice(decoded);

    // Calculate the checksum on the rest
    let calculated = compute(&decoded);

    info!("Calculated {:4x}, Indicated {:4x}", calculated, indicated);

    // Correct?
    if calculated == indicated {
        return Ok(decoded)
    }

    Err(BlockError {
        indicated,
        calculated
    })
}

#[cfg(test)]
mod test {

    use test_log::test;
    use bitvec::prelude::*;
    use crate::lower::block_coder::{block_decode, block_encode};
    use crate::new_bits;

    #[test]
    fn encodes_correctly() {

        let orig = new_bits![
            0, 0, 0, 1, 0, 0, 0, 0,
            1, 0, 1, 1, 0, 0, 0, 0,
            1, 0, 1, 1, 1, 1, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 1, 1,
            0, 0, 0, 0, 0, 1, 1, 1,
            1, 1, 0, 1, 0, 0, 1, 1,
            0, 0, 1, 1
        ];

        let encoded = block_encode(&orig);

        // Should be 16 bits longer than the original
        assert_eq!(encoded.len(), orig.len() + 16);

        // The block code should equal 0xDEF1
        let crc: u16 = encoded[encoded.len() - 16 ..].load_be();
        assert_eq!(crc, 0xDEF1);

    }

    #[test]
    fn decodes_correctly() {

        let orig = new_bits![
            0, 0, 0, 1, 0, 0, 0, 0,
            1, 0, 1, 1, 0, 0, 0, 0,
            1, 0, 1, 1, 1, 1, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 1, 1,
            0, 0, 0, 0, 0, 1, 1, 1,
            1, 1, 0, 1, 0, 0, 1, 1,
            0, 0, 1, 1,

            // checksum
            1, 1, 0, 1, 1, 1, 1, 0,
            1, 1, 1, 1, 0, 0, 0, 1
        ];

        let decoded = block_decode(&orig).unwrap();
        println!("{decoded}");

    }

    #[test]
    fn detects_error() {

        // Same example as above with a bit error
        let orig = new_bits![
            0, 0, 0, 1, 0, 0, 0, 0,
            1, 0, 1, 1, 0, 0, 0, 0,
            1, 0, 1, 1, 1, 1, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 1, 1,
            0, 0, 0, 0, 0, 1, 1, 0,  // <-- final bit flipped
            1, 1, 0, 1, 0, 0, 1, 1,
            0, 0, 1, 1,

            // checksum
            1, 1, 0, 1, 1, 1, 1, 0,
            1, 1, 1, 1, 0, 0, 0, 1
        ];

        let result = block_decode(&orig);
        assert!(result.is_err(), "Expected error, got {:?}", result);

    }
}