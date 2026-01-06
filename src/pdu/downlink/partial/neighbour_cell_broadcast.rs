use crate::codec::{Decodable, Encodable, Reader, Writer};

#[derive(Debug)]
pub struct NeighbourCellBroadcast {
    pub d_nwrk_broadcast_supported: bool,
    pub d_nwrk_enquiry_supported: bool,
}

impl Decodable for NeighbourCellBroadcast {
    fn decode(reader: &mut Reader) -> Self {
        NeighbourCellBroadcast {
            d_nwrk_broadcast_supported: reader.read_bool(),
            d_nwrk_enquiry_supported: reader.read_bool(),
        }
    }
}

impl Encodable for NeighbourCellBroadcast {
    fn encode(&self, writer: &mut Writer) {
        writer.write_bool(self.d_nwrk_enquiry_supported);
        writer.write_bool(self.d_nwrk_enquiry_supported);
    }
}
