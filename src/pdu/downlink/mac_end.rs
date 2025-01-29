use crate::codec::Optional;
use crate::pdu::downlink::partial::{ChannelAllocation, Length, SlotGranting};

pub struct MACEndPDU {
    fill_bits: bool,
    grant_is_on_current_channel: bool,
    length: Length,
    slot_granting: Optional<SlotGranting>,
    channel_allocation: Optional<ChannelAllocation>
}
