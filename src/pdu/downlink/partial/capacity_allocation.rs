use crate::codec::{Cursor, Decodable};

#[derive(Debug)]
pub enum CapacityAllocation {
    FirstSubslot,
    Slots(u32),
    SecondSubslot
}

impl Decodable for CapacityAllocation {
    fn decode(cursor: &mut Cursor) -> Self {
        match cursor.read_int(4) {
            0b0000 => Self::FirstSubslot,
            0b1111 => Self::SecondSubslot,
            slots => Self::Slots(slots)
        }
    }
}
