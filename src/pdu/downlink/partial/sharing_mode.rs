use num_derive::{FromPrimitive, ToPrimitive};
#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum SharingMode {
    ContinuousTransmission = 0b00,
    CarrierSharing = 0b01,
    MCCHSharing = 0b10,
    TrafficCarrierSharing = 0b11
}
