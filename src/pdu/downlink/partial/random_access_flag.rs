use crate::codec::{Reader, Decodable};

#[derive(Debug)]
pub enum RandomAccessFlag {
    Undefined = 0b0,
    Acknowledged = 0b1
}

impl Decodable for RandomAccessFlag {
    fn decode(reader: &mut Reader) -> Self {
        match reader.read_int(1) {
            0b0 => Self::Undefined,
            0b1 => Self::Acknowledged,
            unknown => panic!("unknown random access flag state {unknown}")
        }
    }
}
