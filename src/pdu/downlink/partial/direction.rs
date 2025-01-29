use num_derive::{FromPrimitive, ToPrimitive};
use crate::codec::SizedField;

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum Direction {
    Downlink = 0b01,
    Uplink = 0b10,
    Both = 0b11
}

impl SizedField for Direction {
    fn size() -> usize {
        2
    }
}
