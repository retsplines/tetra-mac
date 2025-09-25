use num_derive::{FromPrimitive, ToPrimitive};
use crate::codec::SizedField;

#[derive(Debug, Eq, PartialEq, ToPrimitive, FromPrimitive)]
enum DownlinkMACPDUType {
    MACResource = 0b00,
    Fragment = 0b01,
    Broadcast = 0b10
}

impl SizedField for DownlinkMACPDUType {
    fn size() -> usize {
        2
    }
}

#[derive(Debug, Eq, PartialEq, ToPrimitive, FromPrimitive)]
enum BroadcastPDUType {
    Sysinfo = 0b00
}

impl SizedField for BroadcastPDUType {
    fn size() -> usize {
        2
    }
}

pub mod uplink;
pub mod downlink;
