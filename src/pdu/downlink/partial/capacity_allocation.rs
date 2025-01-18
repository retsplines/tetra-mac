use crate::codec::{Reader, Decodable};

#[derive(Debug)]
pub enum CapacityAllocation {
    FirstSubslot,
    Slots(u32),
    SecondSubslot
}

impl Decodable for CapacityAllocation {
    fn decode(reader: &mut Reader) -> Self {
        match reader.read_int(4) {
            0b0000 => Self::FirstSubslot,
            0b1111 => Self::SecondSubslot,
            slots => Self::Slots(slots)
        }
    }
}
