use crate::codec::{Reader, Decodable};

mod uplink;
mod downlink;

#[derive(Debug)]
#[derive(Eq, PartialEq)]
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
