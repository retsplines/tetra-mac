use crate::codec::{Decodable, Encodable, Reader, Writer};
use crate::pdu::downlink::BSServiceDetails;

#[derive(Debug)]
pub struct MLESysinfoPDU {
    location_area: u32,
    subscriber_class: u32, // todo: subscriber class is a bitmap
    bs_service_details: BSServiceDetails
}


impl Decodable for MLESysinfoPDU {
    fn decode(reader: &mut Reader) -> Self {
        MLESysinfoPDU {
            location_area: reader.read_int(14),
            subscriber_class: reader.read_int(16),
            bs_service_details: BSServiceDetails::decode(reader)
        }
    }
}

impl Encodable for MLESysinfoPDU {
    fn encode(&self, writer: &mut Writer) {
        writer.write_int(self.location_area, 14);
        writer.write_int(self.subscriber_class, 16);
        self.bs_service_details.encode(writer);
    }
}

