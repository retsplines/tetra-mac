use num_derive::{FromPrimitive, ToPrimitive};
use crate::codec::{Builder, Encodable, SizedField};

#[derive(FromPrimitive, ToPrimitive)]
enum AccessCode {
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
enum BaseFrameLength {
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

struct AccessField {
    access_code: AccessCode,
    base_frame_length: BaseFrameLength
}

impl Encodable for AccessField {
    fn encode(&self, builder: &mut Builder) {
        self.access_code.encode(builder);
        self.base_frame_length.encode(builder);
    }
}

enum DownlinkUsageMarker {
    Reserved,
    CommonControl,
    AssignedControl,
    Unallocated,
    Traffic(u32)
}

impl Encodable for DownlinkUsageMarker {
    fn encode(&self, builder: &mut Builder) {
        builder.write_int(match self {
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

enum UplinkUsageMarker {
    Unallocated,
    Traffic(u32)
}

impl Encodable for UplinkUsageMarker {
    fn encode(&self, builder: &mut Builder) {
        builder.write_int(match self {
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
enum AccessAssignNormalFrame {
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
    fn encode(&self, builder: &mut Builder) {
        match self {
            AccessAssignNormalFrame::DownlinkCommonUplinkCommon { access_field_1, access_field_2} => {
                builder.write_int(0b00, 2);
                access_field_1.encode(builder);
                access_field_2.encode(builder);
            },
            AccessAssignNormalFrame::DownlinkDefinedUplinkCommonAndAssigned { downlink_usage_marker, access_field } => {
                builder.write_int(0b01, 2);
                downlink_usage_marker.encode(builder);
                access_field.encode(builder);
            },
            AccessAssignNormalFrame::DownlinkDefinedUplinkAssignedOnly { downlink_usage_marker, access_field } => {
                builder.write_int(0b10, 2);
                downlink_usage_marker.encode(builder);
                access_field.encode(builder);
            },
            AccessAssignNormalFrame::DownlinkDefinedUplinkDefined { downlink_usage_marker, uplink_usage_marker } => {
                builder.write_int(0b11, 2);
                downlink_usage_marker.encode(builder);
                uplink_usage_marker.encode(builder);
            }
        }
    }
}

/// ACCESS-ASSIGN, as sent in the Control Frame (Frame 18)
/// During the Control Frame, downlink is always for common control, so these options are omitted
enum AccessAssignControlFrame {
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
    fn encode(&self, builder: &mut Builder) {
        match self {
            AccessAssignControlFrame::UplinkCommonOnly { access_field_1, access_field_2} => {
                builder.write_int(0b00, 2);
                access_field_1.encode(builder);
                access_field_2.encode(builder);
            },
            AccessAssignControlFrame::UplinkCommonAndAssigned { access_field_1, access_field_2 } => {
                builder.write_int(0b01, 2);
                access_field_1.encode(builder);
                access_field_2.encode(builder);
            },
            AccessAssignControlFrame::UplinkAssignedOnly { access_field_1, access_field_2 } => {
                builder.write_int(0b10, 2);
                access_field_1.encode(builder);
                access_field_2.encode(builder);
            },
            AccessAssignControlFrame::UplinkCommonAndAssignedTraffic { access_field, uplink_usage_marker } => {
                builder.write_int(0b11, 2);
                access_field.encode(builder);
                uplink_usage_marker.encode(builder);
            }
        }
    }
}

enum AccessAssign {
    NormalFrame(AccessAssignNormalFrame),
    ControlFrame(AccessAssignControlFrame)
}

impl Encodable for AccessAssign {
    fn encode(&self, builder: &mut Builder) {
        match self {
            AccessAssign::NormalFrame(access_assign) => access_assign.encode(builder),
            AccessAssign::ControlFrame(access_assign) => access_assign.encode(builder)
        }
    }
}


mod test {
    use bitvec::prelude::*;
    use super::*;

    #[test]
    fn it_encodes_correctly() {

        let access_assign = AccessAssign::NormalFrame(AccessAssignNormalFrame::DownlinkDefinedUplinkCommonAndAssigned {
            downlink_usage_marker: DownlinkUsageMarker::CommonControl,
            access_field: AccessField {
                access_code: AccessCode::AccessCodeA,
                base_frame_length: BaseFrameLength::Subslots4,
            },
        });

        let mut builder = Builder::new();
        access_assign.encode(&mut builder);
        let bits = builder.done();

        assert_eq!(bits, bits![u8, Msb0;
            0,1,  0,0,0,0,1,0,  0,0,   0,1,1,0
        ]);

    }
}