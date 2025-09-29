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
mod bits;
mod hard_bits;

use bitvec::prelude::*;

fn main() {

    env_logger::init();
    
    let mut bits = new_bits![];

    // Set an arbitrary sized int in the first 8 bits
    bits.push(true);
    bits.push(false);
    bits.push(false);
    bits.push(false);

    // Show the bit representation
    dbg!(&bits);
    dbg!(bits[0]);
}

