use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;
use crate::codec::{Decodable, Encodable, SizedField};

mod uplink;
mod downlink;

#[derive(Debug, Eq, PartialEq, ToPrimitive, FromPrimitive)]
enum DownlinkMACPDUType {
    MACResource
}

impl SizedField for DownlinkMACPDUType {
    fn size() -> usize {
        2
    }
}