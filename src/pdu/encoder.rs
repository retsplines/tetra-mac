use crate::pdu::cursor::*;

/// Functionality for decoding a PDU
pub trait Decodable {
    fn decode(cursor: &mut Cursor) -> Self;
}
