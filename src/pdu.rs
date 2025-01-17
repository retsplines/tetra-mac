use crate::codec::{Cursor, Decodable};

mod uplink;
mod downlink;

#[derive(Debug)]
#[derive(Eq, PartialEq)]
enum DownlinkMACPDUType {
    MACResource
}

impl Decodable for DownlinkMACPDUType {
    fn decode(cursor: &mut Cursor) -> Self {
        match cursor.read_int(2) {
            0b00 => DownlinkMACPDUType::MACResource,
            unknown => panic!("Unknown downlink MAC PDU type {unknown}")
        }
    }
}
