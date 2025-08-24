use bitvec::bitvec;
use bitvec::prelude::BitVec;
use crc::Crc;

/// Implements the (K1+16, K1) block code specified in ETSI EN 300 392-2 §8.2.3.3
/// Effectively appends a CRC to the block.
pub fn encode(block: &BitVec) -> BitVec {
    todo!()
}

/// Decode the (K1+16, K1) block code specified in ETSI EN 300 392-2 §8.2.3.3
pub fn decode(block: &BitVec) -> Result<BitVec, &'static str> {
    todo!()
}