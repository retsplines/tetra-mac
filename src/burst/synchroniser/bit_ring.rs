use bitvec::vec::BitVec;
use crate::bits::Bits;

struct BitRing {
    buffer: Bits,
    head: usize,
    length: usize,
}

impl BitRing {

    pub fn new(capacity: usize) -> Self {
        BitRing {
            buffer: BitVec::with_capacity(capacity),
            head: 0,
            length: 0,
        }
    }

    pub fn push(&mut self, bits: Bits) {

    }

}