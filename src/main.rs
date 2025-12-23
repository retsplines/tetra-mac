#![allow(dead_code)]
#[allow(unused_imports)]
mod codec;
mod upper_mac;
mod tdma_time;
mod burst;
mod logical_channels;
mod pdu;
mod mcch;
mod dqpsk;
mod lower;
mod bits;
mod lower_mac;
mod bnch_helper;
mod aach_helper;

use bitvec::prelude::*;
use crate::burst::DownlinkBurst;
use crate::tdma_time::TDMATime;

fn main() {

    env_logger::init();

    // Arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <slot count>", args[0]);
        std::process::exit(1);
    }

    let slot_count: i32 = args[1].parse().unwrap();

    let mut time = TDMATime::at(0, 17, 0, 0);

    // Generate the first n slots
    for f in 0..slot_count {

        println!("Slot {f} TDMA Time {time:?}");

        let next_burst = lower_mac::generate_dl_slot(&time);
        match next_burst {
            DownlinkBurst::Sync(burst) => {
                println!("SyncBurst: {burst:?}")
            }
            DownlinkBurst::Normal(burst) => {
                println!("NormalBurst: {burst:?}")
            }
        }

        time = time.next();
    }

}

