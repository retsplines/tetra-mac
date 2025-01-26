use num_derive::ToPrimitive;
use num_traits::ToPrimitive;
use crate::codec::{Reader, Decodable, Encodable, Builder};

mod uplink;
mod downlink;

#[derive(Debug, Eq, PartialEq, ToPrimitive)]
enum DownlinkMACPDUType {
    MACResource
}

impl Decodable for DownlinkMACPDUType {
    fn decode(reader: &mut Reader) -> Self {
        match reader.read_int(2) {
            0b00 => DownlinkMACPDUType::MACResource,
            unknown => panic!("Unknown downlink MAC PDU type {unknown}")
        }
    }
}

impl Encodable for DownlinkMACPDUType {
    fn encode(&self, builder: &mut Builder) {
        builder.write_int(self.to_u32().unwrap(), 2);
    }
}
