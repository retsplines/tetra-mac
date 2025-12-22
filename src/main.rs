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

fn main() {

    env_logger::init();

    // Generate the next frame
    let mac = mac::MAC::new();
    let next_frame = mac.generate_next();
    println!("{next_frame:?}");

}

