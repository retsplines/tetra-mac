use crate::codec::{Reader, Decodable, Encodable, Builder};

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
    fn decode(reader: &mut Reader) -> Self {
        let address_type_field = reader.read_int(3);
        match address_type_field {
            0b000 => Address::NullPDU,
            0b001 => Address::SSI { address: reader.read_int(24) },
            0b010 => Address::EventLabel { event_label: reader.read_int(10) },
            0b011 => Address::USSI { ussi: reader.read_int(24) },
            0b100 => Address::SMI { smi: reader.read_int(24) },
            0b101 => Address::SSIPlusEventLabel {
                ssi: reader.read_int(24),
                event_label: reader.read_int(10)
            },
            0b110 => Address::SSIPlusUsageMarker {
                ssi: reader.read_int(24),
                usage_marker: reader.read_int(10)
            },
            0b111 => Address::SMIPlusEventLabel {
                smi: reader.read_int(24),
                event_label: reader.read_int(10)
            },
            unknown => panic!("unknown address type {unknown}")
        }
    }
}

impl Encodable for Address {
    fn encode(&self, builder: &mut Builder) {
        todo!()
    }
}