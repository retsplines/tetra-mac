use num_derive::{FromPrimitive, ToPrimitive};
use crate::codec::SizedField;

#[derive(Debug, Eq, PartialEq, ToPrimitive, FromPrimitive)]
enum DownlinkMACPDUType {
    MACResource
}

impl SizedField for DownlinkMACPDUType {
    fn size() -> usize {
        2
    }
}

pub mod uplink;
pub mod downlink;
