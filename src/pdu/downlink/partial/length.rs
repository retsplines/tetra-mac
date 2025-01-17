use bitvec::macros::internal::funty::Fundamental;
use crate::codec::{Cursor, Decodable};

#[derive(Debug)]
pub enum Length {
    Reserved,
    NullPDU,
    Octets(usize),
    SecondHalfSlotStolen,
    StartOfFragmentation
}

impl Decodable for Length {
    fn decode(cursor: &mut Cursor) -> Self {
        let length_field = cursor.read_int(6);
        match length_field {
            0b000000 | 0b000001 => Self::Reserved,
            0b000010 => Self::NullPDU,
            0b000011 => Self::Reserved,
            0b100011..=0b111101 => Self::Reserved,
            0b111110 => Self::SecondHalfSlotStolen,
            0b111111 => Self::StartOfFragmentation,
            octets => Self::Octets(octets.as_usize()),
        }
    }
}
