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
use bitvec::prelude::*;
use crate::bits::from_bitstr;

fn main() {

    env_logger::init();
    
    let mut bits = from_bitstr("");

    // Set an arbitrary sized int in the first 8 bits
    bits.push(true);
    bits.push(false);
    bits.push(false);
    bits.push(false);

    // Show the bit representation
    dbg!(&bits);
    dbg!(bits[0]);
}

