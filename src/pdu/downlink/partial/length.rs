use bitvec::macros::internal::funty::Fundamental;
use crate::codec::{Reader, Decodable, Encodable, Writer};

const LENGTH_SIZE: usize = 6;

#[derive(Debug)]
pub enum Length {
    Reserved,
    NullPDU,
    Octets(usize),
    SecondHalfSlotStolen,
    StartOfFragmentation
}

impl Decodable for Length {
    fn decode(reader: &mut Reader) -> Self {
        let length_field = reader.read_int(LENGTH_SIZE);
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

impl Encodable for Length {
    fn encode(&self, writer: &mut Writer) {
        writer.write_int(match self {
            Self::Reserved => 0b000000,
            Self::NullPDU => 0b000010,
            Length::Octets(octets) => octets.as_u32(),
            Length::SecondHalfSlotStolen => 0b111110,
            Length::StartOfFragmentation => 0b111111
        }, LENGTH_SIZE);
    }
}
