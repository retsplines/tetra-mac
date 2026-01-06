use num_derive::{FromPrimitive, ToPrimitive};
use crate::codec::{Writer, Encodable, SizedField};

#[derive(FromPrimitive, ToPrimitive)]
pub enum AccessCode {
    AccessCodeA = 0b00,
    AccessCodeB = 0b01,
    AccessCodeC = 0b10,
    AccessCodeD = 0b11
}

impl SizedField for AccessCode {
    fn size() -> usize {
        2
    }
}

#[derive(FromPrimitive, ToPrimitive)]
pub enum BaseFrameLength {
    ReservedSubslot = 0b0000,
    CLCHSubslot = 0b0001,
    OngoingFrame = 0b0010,
    Subslots1 = 0b0011,
    Subslots2,
    Subslots3,
    Subslots4,
    Subslots5,
    Subslots6,
    Subslots8,
    Subslots10,
    Subslots12,
    Subslots16,
    Subslots20,
    Subslots24,
    Subslots32
}

impl SizedField for BaseFrameLength {
    fn size() -> usize {
        4
    }
}

pub struct AccessField {
    pub access_code: AccessCode,
    pub base_frame_length: BaseFrameLength
}

impl Encodable for AccessField {
    fn encode(&self, writer: &mut Writer) {
        self.access_code.encode(writer);
        self.base_frame_length.encode(writer);
    }
}

pub enum DownlinkUsageMarker {
    Reserved,
    CommonControl,
    AssignedControl,
    Unallocated,
    Traffic(u32)
}

impl Encodable for DownlinkUsageMarker {
    fn encode(&self, writer: &mut Writer) {
        writer.write_int(match self {
            DownlinkUsageMarker::Reserved => panic!("the UMr usage marker is reserved"),
            DownlinkUsageMarker::CommonControl => 0b000010,
            DownlinkUsageMarker::AssignedControl => 0b000001,
            DownlinkUsageMarker::Unallocated => 0b000000,
            DownlinkUsageMarker::Traffic(traffic_um) => {
                if *traffic_um <= 0b000011 {
                    panic!("the downlink traffic usage marker may not be one of UMr, UMc, UMa or UMx")
                }
                *traffic_um
            }
        }, 6)
    }
}

pub enum UplinkUsageMarker {
    Unallocated,
    Traffic(u32)
}

impl Encodable for UplinkUsageMarker {
    fn encode(&self, writer: &mut Writer) {
        writer.write_int(match self {
            UplinkUsageMarker::Unallocated => 0b000000,
            UplinkUsageMarker::Traffic(traffic_um) => {
                if *traffic_um <= 0b000011 {
                    panic!("the uplink traffic usage marker may not be one of UMr, UMc, UMa or UMx")
                }
                *traffic_um
            }
        }, 6)
    }
}

/// ACCESS-ASSIGN, as sent in a normal frame (Frames 1-17)
pub enum AccessAssignNormalFrame {
    DownlinkCommonUplinkCommon {
        access_field_1: AccessField,
        access_field_2: AccessField
    },
    DownlinkDefinedUplinkCommonAndAssigned {
        downlink_usage_marker: DownlinkUsageMarker,
        access_field: AccessField
    },
    DownlinkDefinedUplinkAssignedOnly {
        downlink_usage_marker: DownlinkUsageMarker,
        access_field: AccessField
    },
    DownlinkDefinedUplinkDefined {
        downlink_usage_marker: DownlinkUsageMarker,
        uplink_usage_marker: UplinkUsageMarker
    }
}

impl Encodable for AccessAssignNormalFrame {
    fn encode(&self, writer: &mut Writer) {
        match self {
            AccessAssignNormalFrame::DownlinkCommonUplinkCommon { access_field_1, access_field_2} => {
                writer.write_int(0b00, 2);
                access_field_1.encode(writer);
                access_field_2.encode(writer);
            },
            AccessAssignNormalFrame::DownlinkDefinedUplinkCommonAndAssigned { downlink_usage_marker, access_field } => {
                writer.write_int(0b01, 2);
                downlink_usage_marker.encode(writer);
                access_field.encode(writer);
            },
            AccessAssignNormalFrame::DownlinkDefinedUplinkAssignedOnly { downlink_usage_marker, access_field } => {
                writer.write_int(0b10, 2);
                downlink_usage_marker.encode(writer);
                access_field.encode(writer);
            },
            AccessAssignNormalFrame::DownlinkDefinedUplinkDefined { downlink_usage_marker, uplink_usage_marker } => {
                writer.write_int(0b11, 2);
                downlink_usage_marker.encode(writer);
                uplink_usage_marker.encode(writer);
            }
        }
    }
}

/// ACCESS-ASSIGN, as sent in the Control Frame (Frame 18)
/// During the Control Frame, downlink is always for common control, so these options are omitted
pub enum AccessAssignControlFrame {
    UplinkCommonOnly {
        access_field_1: AccessField,
        access_field_2: AccessField
    },
    UplinkCommonAndAssigned {
        access_field_1: AccessField,
        access_field_2: AccessField
    },
    UplinkAssignedOnly {
        access_field_1: AccessField,
        access_field_2: AccessField
    },
    UplinkCommonAndAssignedTraffic {
        uplink_usage_marker: UplinkUsageMarker,
        access_field: AccessField
    }
}

impl Encodable for AccessAssignControlFrame {
    fn encode(&self, writer: &mut Writer) {
        match self {
            AccessAssignControlFrame::UplinkCommonOnly { access_field_1, access_field_2} => {
                writer.write_int(0b00, 2);
                access_field_1.encode(writer);
                access_field_2.encode(writer);
            },
            AccessAssignControlFrame::UplinkCommonAndAssigned { access_field_1, access_field_2 } => {
                writer.write_int(0b01, 2);
                access_field_1.encode(writer);
                access_field_2.encode(writer);
            },
            AccessAssignControlFrame::UplinkAssignedOnly { access_field_1, access_field_2 } => {
                writer.write_int(0b10, 2);
                access_field_1.encode(writer);
                access_field_2.encode(writer);
            },
            AccessAssignControlFrame::UplinkCommonAndAssignedTraffic { access_field, uplink_usage_marker } => {
                writer.write_int(0b11, 2);
                access_field.encode(writer);
                uplink_usage_marker.encode(writer);
            }
        }
    }
}

pub enum AccessAssign {
    NormalFrame(AccessAssignNormalFrame),
    ControlFrame(AccessAssignControlFrame)
}

impl Encodable for AccessAssign {
    fn encode(&self, writer: &mut Writer) {
        match self {
            AccessAssign::NormalFrame(access_assign) => access_assign.encode(writer),
            AccessAssign::ControlFrame(access_assign) => access_assign.encode(writer)
        }
    }
}

#[cfg(test)]
mod tests {
    
    use crate::bits::from_bitstr;
    use super::*;

    #[test]
    fn encodes() {

        let access_assign = AccessAssign::NormalFrame(AccessAssignNormalFrame::DownlinkDefinedUplinkCommonAndAssigned {
            downlink_usage_marker: DownlinkUsageMarker::CommonControl,
            access_field: AccessField {
                access_code: AccessCode::AccessCodeA,
                base_frame_length: BaseFrameLength::Subslots4,
            },
        });

        let mut writer = Writer::new();
        access_assign.encode(&mut writer);
        let bits = writer.done();

        assert_eq!(bits, from_bitstr("01000010000110"));

    }
}