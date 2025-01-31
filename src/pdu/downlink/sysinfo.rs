use num_derive::{FromPrimitive, ToPrimitive};
use crate::codec::{Builder, Encodable, SizedField};
use crate::pdu::downlink::partial::{Offset, Timeslots};

enum HyperframeOrCipherKey {
    Hyperframe {
        hyperframe_number: u32
    },
    CipherKey {
        cck_id_or_key_version_number: u32
    }
}

impl Encodable for HyperframeOrCipherKey {
    fn encode(&self, builder: &mut Builder) {
        match self {
            HyperframeOrCipherKey::Hyperframe { hyperframe_number: u32 @ hyperframe } => {
                builder.write_bool(false);
                builder.write_int(*hyperframe, 16);
            },
            HyperframeOrCipherKey::CipherKey { cck_id_or_key_version_number: u32 @ cck_id } => {
                builder.write_bool(true);
                builder.write_int(*cck_id, 16);
            }
        }
    }
}

type TSModeBitmap = [bool; 20];

enum Immediate {
    AlwaysRandomise,
    AfterFrames(u32)
}

enum TimeslotPointer {
    SameAsDownlink,
    InTimeslots(Timeslots)
}

struct AccessCodeDefinition {
    immediate: Immediate,
    waiting_time_opportunities: u32,
    number_of_attempts: u32,
    frame_length_x4: bool,
    timeslot: TimeslotPointer,
    minimum_priority: u32
}

enum OptionalField {
    TSModeEvenMultiframe (TSModeBitmap),
    TSModeOddMultiframe (TSModeBitmap),
    DefaultAccessCodeA,
    ExtendedServicesBroadcast
}

#[derive(FromPrimitive, ToPrimitive)]
enum NumberOfCommonSCCH {
    None = 0b00,
    Timeslot2 = 0b01,
    Timeslot23 = 0b10,
    Timeslot234 = 0b11
}

impl SizedField for NumberOfCommonSCCH {
    fn size() -> usize {
        2
    }
}

struct RFParameters {
    ms_txpwr_max_cell: u32,
    rxlev_access_min: u32,
    access_parameter: u32,
    radio_downlink_timeout: u32
}

impl Encodable for RFParameters {
    fn encode(&self, builder: &mut Builder) {
        builder.write_int(self.ms_txpwr_max_cell, 3);
        builder.write_int(self.rxlev_access_min, 4);
        builder.write_int(self.access_parameter, 4);
        builder.write_int(self.radio_downlink_timeout, 4);
    }
}

struct Sysinfo {
    main_carrier: u32,
    frequency_band: u32,
    offset: Offset,
    duplex_spacing: u32,
    reverse: bool,
    number_of_common_scch: NumberOfCommonSCCH,
    rf_parameters: RFParameters,
    hyperframe_or_cipher_key: HyperframeOrCipherKey,
    optional_field: OptionalField
}

impl Encodable for Sysinfo {
    fn encode(&self, builder: &mut Builder) {
        builder.write_int(self.main_carrier, 12);
        builder.write_int(self.frequency_band, 4);
        self.offset.encode(builder);
        builder.write_int(self.duplex_spacing, 3);
        builder.write_bool(self.reverse);
        self.number_of_common_scch.encode(builder);
        self.rf_parameters.encode(builder);
        self.hyperframe_or_cipher_key.encode(builder);
        self.optional_field.encode(builder);
    }
}
