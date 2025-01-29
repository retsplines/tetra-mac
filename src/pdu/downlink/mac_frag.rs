use bitvec::macros::internal::funty::Fundamental;
use bitvec::prelude::*;
use crate::codec::{Reader, Decodable, Encodable, Optional, Builder};
use crate::pdu::downlink::partial::{ChannelAllocation, Length, SlotGranting};
use crate::pdu::DownlinkMACPDUType;

#[derive(Debug)]
pub struct MACFragPDU {
    fill_bits: bool
}
