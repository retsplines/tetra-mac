use crate::codec::{Reader, Decodable};
use super::granting_delay::GrantingDelay;
use super::capacity_allocation::CapacityAllocation;

#[derive(Debug)]
pub struct SlotGranting {
    capacity_allocation: CapacityAllocation,
    granting_delay: GrantingDelay
}

impl Decodable for SlotGranting {
    fn decode(reader: &mut Reader) -> Self {
        SlotGranting {
            capacity_allocation: CapacityAllocation::decode(reader),
            granting_delay: GrantingDelay::decode(reader)
        }
    }
}
