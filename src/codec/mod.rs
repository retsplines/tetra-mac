mod cursor;

pub use cursor::Cursor;
use crate::codec::Optional::{Absent, Present};

#[derive(Debug)]
pub enum Optional<T> {
    Absent,
    Present(T)
}

impl <T> Decodable for Optional<T> where T: Decodable {
    fn decode(cursor: &mut Cursor) -> Self {
        if cursor.read_bool() {
            Present(T::decode(cursor))
        } else {
            Absent
        }
    }
}

impl <T> Encodable for Optional<T> where T: Encodable {
    /// Encode the field, including a prefix O-bit
    fn encode(&self, cursor: &mut Cursor) -> usize {
        match self {
            Present(value) => value.encode(cursor),
            Absent => cursor.write_bool(false)
        }
    }
}

/// Functionality for decoding a PDU from an existing Cursor
pub trait Decodable {
    fn decode(cursor: &mut Cursor) -> Self;

}

/// Functionality for encoding a PDU into an existing Cursor
pub trait Encodable {
    fn encode(&self, cursor: &mut Cursor) -> usize;
}

