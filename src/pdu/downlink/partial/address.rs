use crate::codec::{Cursor, Decodable};

#[derive(Debug)]
pub enum Address {
    NullPDU,
    SSI { address: u32 },
    EventLabel { event_label: u32 },
    USSI { ussi: u32 },
    SMI { smi: u32 },
    SSIPlusEventLabel { ssi: u32, event_label: u32 },
    SSIPlusUsageMarker { ssi: u32, usage_marker: u32 },
    SMIPlusEventLabel { smi: u32, event_label: u32 }
}

impl Decodable for Address {
    fn decode(cursor: &mut Cursor) -> Self {
        let address_type_field = cursor.read_int(3);
        match address_type_field {
            0b000 => Address::NullPDU,
            0b001 => Address::SSI { address: cursor.read_int(24) },
            0b010 => Address::EventLabel { event_label: cursor.read_int(10) },
            0b011 => Address::USSI { ussi: cursor.read_int(24) },
            0b100 => Address::SMI { smi: cursor.read_int(24) },
            0b101 => Address::SSIPlusEventLabel {
                ssi: cursor.read_int(24),
                event_label: cursor.read_int(10)
            },
            0b110 => Address::SSIPlusUsageMarker {
                ssi: cursor.read_int(24),
                usage_marker: cursor.read_int(10)
            },
            0b111 => Address::SMIPlusEventLabel {
                smi: cursor.read_int(24),
                event_label: cursor.read_int(10)
            },
            unknown => panic!("unknown address type {unknown}")
        }
    }
}