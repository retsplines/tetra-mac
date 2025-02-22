mod reader;
mod builder;
mod fill_bits;

pub use reader::Reader;
pub use builder::Builder;

use crate::codec::Optional::{Absent, Present};

#[derive(Debug)]
pub enum Optional<T> {
    Absent,
    Present(T)
}

impl <T> Decodable for Optional<T> where T: Decodable {
    fn decode(reader: &mut Reader) -> Self {
        if reader.read_bool() {
            Present(T::decode(reader))
        } else {
            Absent
        }
    }
}

impl <T> Encodable for Optional<T> where T: Encodable {
    /// Encode the field, including a prefix O-bit
    fn encode(&self, builder: &mut Builder) {
        match self {
            Present(value) => value.encode(builder),
            Absent => builder.write_bool(false)
        }
    }
}

/// Functionality for decoding a PDU from an existing reader
pub trait Decodable {
    fn decode(reader: &mut Reader) -> Self;
}

/// Functionality for encoding a PDU into an existing reader
pub trait Encodable {
    fn encode(&self, builder: &mut Builder);
}

pub trait SizedField {
    fn size() -> usize;
}

// Implement Encodable and Decodable for Sized ToPrimitive types
// This deals with any enum fields that can be directly represented as an integer (i.e. have no
// special encoding/decoding rules)
impl <T> Encodable for T where T: num::ToPrimitive + SizedField {
    fn encode(&self, builder: &mut Builder) {
        builder.write_int(num::ToPrimitive::to_u32(self).unwrap(), Self::size());
    }
}

impl <T> Decodable for T where T: num::FromPrimitive + SizedField {
    fn decode(reader: &mut Reader) -> Self {
        num::FromPrimitive::from_u32(reader.read_int(Self::size())).unwrap()
    }
}