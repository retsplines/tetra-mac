use crate::codec::{Reader, Decodable, Encodable, Writer};
use super::{AllocationType, Direction, ExtendedCarrierNumbering, MonitoringPatterns, TimeslotAssigned};

#[derive(Debug)]
pub struct ChannelAllocation {
    allocation_type: AllocationType,
    timeslot_assigned: TimeslotAssigned,
    direction: Direction,
    clch_permission: bool,
    cell_change: bool,
    carrier_number: u32,
    extended_carrier_numbering: Option<ExtendedCarrierNumbering>,
    reverse_operation: bool,
    monitoring_pattern: MonitoringPatterns,
    frame_18_monitoring_pattern: MonitoringPatterns
}

impl Decodable for ChannelAllocation {
    fn decode(reader: &mut Reader) -> Self {
        ChannelAllocation {
            allocation_type: num::FromPrimitive::from_u32(reader.read_int(2)).unwrap(),
            timeslot_assigned: TimeslotAssigned::decode(reader),
            direction: Direction::Downlink,
            clch_permission: false,
            cell_change: false,
            carrier_number: 0,
            extended_carrier_numbering: None,
            reverse_operation: false,
            monitoring_pattern: MonitoringPatterns::None,
            frame_18_monitoring_pattern: MonitoringPatterns::None,
        }
    }
}

impl Encodable for ChannelAllocation {
    fn encode(&self, writer: &mut Writer) {
        self.allocation_type.encode(writer);
        self.timeslot_assigned.encode(writer);
        self.direction.encode(writer);
    }
}