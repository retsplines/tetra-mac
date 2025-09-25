use crate::codec::{Decodable, Encodable, Reader, Writer};

#[derive(Debug)]
pub enum CellServiceLevel {
    CellLoadUnknown,
    LowCellLoad,
    MediumCellLoad,
    HighCellLoad,
}

impl Decodable for CellServiceLevel {
    fn decode(reader: &mut Reader) -> Self {
        match reader.read_int(2) {
            0b00 => Self::CellLoadUnknown,
            0b01 => Self::LowCellLoad,
            0b10 => Self::MediumCellLoad,
            0b11 => Self::HighCellLoad,
            _ => panic!("Invalid value for CellServiceLevel"),
        }
    }
}

impl Encodable for CellServiceLevel {
    fn encode(&self, writer: &mut Writer) {
        writer.write_int(match self {
            Self::CellLoadUnknown => 0b00,
            Self::LowCellLoad => 0b01,
            Self::MediumCellLoad => 0b10,
            Self::HighCellLoad => 0b11,
        }, 2);
    }
}
