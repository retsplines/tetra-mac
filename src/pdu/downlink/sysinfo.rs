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
            HyperframeOrCipherKey::Hyperframe { hyperframe_number: hyperframe } => {
                builder.write_bool(false);
                builder.write_int(*hyperframe, 16);
            },
            HyperframeOrCipherKey::CipherKey { cck_id_or_key_version_number: cck_id } => {
                builder.write_bool(true);
                builder.write_int(*cck_id, 16);
            }
        }
    }
}

type TSModeBitmap = [bool; 20];

impl Encodable for TSModeBitmap {
    fn encode(&self, builder: &mut Builder) {
        for bit in self.iter() {
            builder.write_bool(*bit);
        }
    }
}

enum Immediate {
    AlwaysRandomise,
    AfterFrames(u32),
    Immediate
}

impl Encodable for Immediate {
    fn encode(&self, builder: &mut Builder) {
        match self {
            Immediate::AlwaysRandomise => builder.write_int(0b0000, 4),
            Immediate::Immediate => builder.write_int(0b1111, 4),
            Immediate::AfterFrames(after) => builder.write_int(*after, 4)
        }
    }
}

enum TimeslotPointer {
    SameAsDownlink,
    InTimeslots(Timeslots)
}

impl Encodable for TimeslotPointer {
    fn encode(&self, builder: &mut Builder) {
        match self {
            TimeslotPointer::SameAsDownlink => builder.write_int(0b0000, 4),
            TimeslotPointer::InTimeslots(timeslots) => timeslots.encode(builder)
        }
    }
}

struct AccessCodeDefinition {
    immediate: Immediate,
    waiting_time_opportunities: u32,
    number_of_attempts: u32,
    frame_length_x4: bool,
    timeslot: TimeslotPointer,
    minimum_priority: u32
}

impl Encodable for AccessCodeDefinition {
    fn encode(&self, builder: &mut Builder) {
        self.immediate.encode(builder);
        builder.write_int(self.waiting_time_opportunities, 4);
        builder.write_int(self.number_of_attempts, 4);
        builder.write_bool(self.frame_length_x4);
        self.timeslot.encode(builder);
        builder.write_int(self.minimum_priority, 3);
    }
}

struct ExtendedServicesBroadcast { }

impl Encodable for ExtendedServicesBroadcast {
    fn encode(&self, _builder: &mut Builder) {
        unimplemented!("extended services broadcast is not yet supported");
    }
}

enum OptionalField {
    TSModeEvenMultiframe(TSModeBitmap),
    TSModeOddMultiframe(TSModeBitmap),
    DefaultAccessCodeA(AccessCodeDefinition),
    ExtendedServicesBroadcast(ExtendedServicesBroadcast)
}

impl Encodable for OptionalField {
    fn encode(&self, builder: &mut Builder) {
        match self {
            OptionalField::TSModeEvenMultiframe(bitmap) => {
                builder.write_int(0b00, 2);
                bitmap.encode(builder);
            },
            OptionalField::TSModeOddMultiframe(bitmap) => {
                builder.write_int(0b01, 2);
                bitmap.encode(builder);
            },
            OptionalField::DefaultAccessCodeA(access_code_definition) => {
                builder.write_int(0b10, 2);
                access_code_definition.encode(builder);
            },
            OptionalField::ExtendedServicesBroadcast(extended_services_broadcast) => {
                builder.write_int(0b11, 2);
                extended_services_broadcast.encode(builder);
            }
        }
    }
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

mod test {

    use super::*;

    #[test]
    fn it_encodes_correctly() {
        let sysinfo = Sysinfo {
            main_carrier: 0x123,
            frequency_band: 0x1,
            offset: Offset::NoOffset,
            duplex_spacing: 0x1,
            reverse: false,
            number_of_common_scch: NumberOfCommonSCCH::Timeslot2,
            rf_parameters: RFParameters {
                ms_txpwr_max_cell: 0x1,
                rxlev_access_min: 0x1,
                access_parameter: 0x1,
                radio_downlink_timeout: 0x1
            },
            hyperframe_or_cipher_key: HyperframeOrCipherKey::Hyperframe {
                hyperframe_number: 0x1
            },
            optional_field: OptionalField::DefaultAccessCodeA(AccessCodeDefinition {
                immediate: Immediate::AlwaysRandomise,
                waiting_time_opportunities: 0,
                number_of_attempts: 0,
                frame_length_x4: false,
                timeslot: TimeslotPointer::SameAsDownlink,
                minimum_priority: 0,
            })
        };

        let mut builder = Builder::new();
        sysinfo.encode(&mut builder);
        let result = builder.done();

        dbg!(result);


    }

}