use bitvec::prelude::{*};
use bitvec::view::BitView;
use log::info;

fn compute(block: &BitVec) -> u16 {

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
pub fn encode(block: &BitVec) -> BitVec {

    let crc = compute(block);

    // Append the CRC bits to the end of the block
    let mut out = block.clone();
    out.extend_from_bitslice(&crc.view_bits::<Lsb0>()[..16]);

    out
}

/// Decode the (K1+16, K1) block code specified in ETSI EN 300 392-2 §8.2.3.3
pub fn decode(block: &BitVec) -> Result<BitVec, &'static str> {

    // Block must be at least 17 bits to be able to be validated
    assert!(block.len() > 17, "require >= 17 bits for block-decode");

    // Strip the final 16 bits which contain the checksum
    let (decoded, crc_bits) = block.split_at(block.len() - 16);
    let indicated_crc: u16 = crc_bits.load();

    // Copy the block bits without the checksum into a new vec
    let decoded = decoded.to_bitvec();

    // Calculate the checksum on the rest
    let calculated_crc = compute(&decoded);

    info!("Calculated {:4x}, Indicated {:4x}", calculated_crc, indicated_crc);

    // Copy the bits into a new vec
    Ok(decoded)
}

#[cfg(test)]
mod test {

    use test_log::test;
    use bitvec::bitvec;
    use bitvec::prelude::*;
    use crate::lower::block_coder::encode;

    #[test]
    fn encodes_correctly() {

        let orig = bitvec![
            0, 0, 0, 1, 0, 0, 0, 0,
            1, 0, 1, 1, 0, 0, 0, 0,
            1, 0, 1, 1, 1, 1, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 1, 1,
            0, 0, 0, 0, 0, 1, 1, 1,
            1, 1, 0, 1, 0, 0, 1, 1,
            0, 0, 1, 1
        ];

        let encoded = encode(&orig);

        // Should be 16 bits longer than the original
        assert_eq!(encoded.len(), orig.len() + 16);

        // The block code should equal 0xDEF1
        let crc: u16 = encoded[encoded.len() - 16 ..].load();
        assert_eq!(crc, 0xDEF1);

    }

    #[test]
    fn decodes_correctly() {

        let orig = bitvec![
            1, 0, 1, 1, 0, 0, 1, 0,
            0, 1, 1, 0, 1, 0, 0, 1,
            1, 1, 0, 0, 0, 1, 0, 1,
            0, 0, 1, 1, 1, 0, 0, 0,
            1, 0, 0, 1, 0, 1, 1, 0,
            0, 1, 0, 0, 1, 1, 1, 0,
            1, 0, 1, 0, 1, 1, 0, 1,
            0, 1, 1, 0,

            // checksum
            1, 0, 1, 1, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0
        ];

        let encoded = encode(&orig);

        println!("{encoded}");


    }
}