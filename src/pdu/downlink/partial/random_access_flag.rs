use crate::codec::{Cursor, Decodable};

#[derive(Debug)]
pub enum RandomAccessFlag {
    Undefined = 0b0,
    Acknowledged = 0b1
}

impl Decodable for RandomAccessFlag {
    fn decode(cursor: &mut Cursor) -> Self {
        match cursor.read_int(1) {
            0b0 => Self::Undefined,
            0b1 => Self::Acknowledged,
            unknown => panic!("unknown random access flag state {unknown}")
        }
    }
}
