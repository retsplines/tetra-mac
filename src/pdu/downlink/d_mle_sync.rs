use crate::codec::{Decodable, Encodable, Reader, Writer};
use crate::pdu::downlink::partial::{NeighbourCellBroadcast, CellServiceLevel, LateEntryInfo};

#[derive(Debug)]
pub struct MLESyncPDU {
    pub mcc: u32,
    pub mnc: u32,
    pub neighbour_cell_broadcast: NeighbourCellBroadcast,
    pub cell_service_level: CellServiceLevel,
    pub late_entry_info: LateEntryInfo
}


impl Decodable for MLESyncPDU {
    fn decode(reader: &mut Reader) -> Self {
        MLESyncPDU {
            mcc: reader.read_int(10),
            mnc: reader.read_int(14),
            neighbour_cell_broadcast: NeighbourCellBroadcast::decode(reader),
            cell_service_level: CellServiceLevel::decode(reader),
            late_entry_info: LateEntryInfo::decode(reader)
        }
    }
}

impl Encodable for MLESyncPDU {
    fn encode(&self, writer: &mut Writer) {
        writer.write_int(self.mcc, 10);
        writer.write_int(self.mnc, 14);
        self.neighbour_cell_broadcast.encode(writer);
        self.cell_service_level.encode(writer);
        self.late_entry_info.encode(writer);
    }
}

