#[allow(dead_code)]
#[allow(unused_imports)]
mod pdu;
mod codec;
mod mac;
mod tdma_time;

use bitvec::prelude::*;
use bitvec::prelude as bv;

// Define a common type for bit storage
type Bits = bv::BitVec<u8, bv::Msb0>;

fn main() {

    let mut bits= bitvec![u8, Msb0; 0; 16];

    // Set an arbitrary sized int in the first 8 bits
    bits[0..8].store(32);

    // Set an arbitrary bit
    bits.set(12, true);

    // Show the bit representation
    dbg!(&bits);

    // Get access to the underlying bytes
    let bytes = bits.as_raw_slice();
    dbg!(bytes);
}

