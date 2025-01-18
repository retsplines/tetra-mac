mod reader;
mod builder;

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
    fn encode(&self, builder: &mut Builder) -> usize {
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
    fn encode(&self, builder: &mut Builder) -> usize;
}

