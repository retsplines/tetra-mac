#![allow(dead_code)]
#[allow(unused_imports)]
mod codec;
mod mac;
mod tdma_time;
mod burst;
mod channels;
mod pdu;
mod mcch;
mod dqpsk;
mod lower;

use bitvec::prelude::*;

// Define a common type for bit storage
type Bits = BitVec<u8, Msb0>;

fn main() {

    env_logger::init();
    
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

