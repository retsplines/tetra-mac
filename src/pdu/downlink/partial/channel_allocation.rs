use crate::codec::{Reader, Decodable};
use super::{AllocationType, TimeslotAssigned};

#[derive(Debug)]
pub struct ChannelAllocation {
    allocation_type: AllocationType,
    timeslot_assigned: TimeslotAssigned,
    // direction: Direction,
    // clch_permission: bool,
    // cell_change: bool,
    // carrier_number: u32,
    // extended_carrier_numbering: Option<ExtendedCarrierNumbering>,
    // reverse_operation: bool,
    // monitoring_pattern: MonitoringPatterns,
    // frame_18_monitoring_pattern: MonitoringPatterns
}

impl Decodable for ChannelAllocation {
    fn decode(reader: &mut Reader) -> Self {
        ChannelAllocation {
            allocation_type: num::FromPrimitive::from_u32(reader.read_int(2)).unwrap(),
            timeslot_assigned: TimeslotAssigned::decode(reader)
        }
    }
}
