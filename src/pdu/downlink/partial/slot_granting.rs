use crate::codec::{Cursor, Decodable};
use super::granting_delay::GrantingDelay;
use super::capacity_allocation::CapacityAllocation;

#[derive(Debug)]
pub struct SlotGranting {
    capacity_allocation: CapacityAllocation,
    granting_delay: GrantingDelay
}

impl Decodable for SlotGranting {
    fn decode(cursor: &mut Cursor) -> Self {
        SlotGranting {
            capacity_allocation: CapacityAllocation::decode(cursor),
            granting_delay: GrantingDelay::decode(cursor)
        }
    }
}
