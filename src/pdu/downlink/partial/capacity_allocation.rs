use crate::codec::{Reader, Decodable, Encodable, Builder};

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

impl Encodable for CapacityAllocation {
    fn encode(&self, builder: &mut Builder) {
        builder.write_int(match self {
            Self::FirstSubslot => 0b0000,
            Self::SecondSubslot => 0b1111,
            Self::Slots(slots) => *slots
        }, 4);
    }
}
