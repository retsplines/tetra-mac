use crate::codec::{Reader, Decodable, Encodable, Writer};

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

impl Address {
    fn get_type(&self) -> u32 {
        match self {
            Address::NullPDU => 0b000,
            Address::SSI { .. } => 0b001,
            Address::EventLabel { .. } => 0b010,
            Address::USSI { .. } => 0b011,
            Address::SMI { .. } => 0b100,
            Address::SSIPlusEventLabel { .. } => 0b101,
            Address::SSIPlusUsageMarker { .. } => 0b110,
            Address::SMIPlusEventLabel { .. } => 0b111
        }
    }
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
    fn encode(&self, writer: &mut Writer) {

        // Write the type
        writer.write_int(self.get_type(), 3);

        // Write the content
        match self {
            Self::NullPDU => (),
            Self::SSI { address } => {
                writer.write_int(*address, 24);
            },
            Self::EventLabel { event_label } => {
                writer.write_int(*event_label, 10);
            },
            Self::USSI { ussi } => {
                writer.write_int(*ussi, 24);
            },
            Self::SMI { smi } => {
                writer.write_int(*smi, 24);
            },
            Self::SSIPlusEventLabel { ssi, event_label } => {
                writer.write_int(*ssi, 24);
                writer.write_int(*event_label, 10);
            },
            Self::SSIPlusUsageMarker { ssi, usage_marker } => {
                writer.write_int(*ssi, 24);
                writer.write_int(*usage_marker, 10);
            },
            Self::SMIPlusEventLabel { smi, event_label } => {
                writer.write_int(*smi, 24);
                writer.write_int(*event_label, 10);
            }
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn encodes() {
        let address = Address::SSI { address: 0xFFFFFE };
        let mut writer = Writer::new();
        address.encode(&mut writer);
        let bits = writer.done();
        dbg!(bits);
    }


}