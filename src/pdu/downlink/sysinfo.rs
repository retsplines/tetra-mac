use num_derive::{FromPrimitive, ToPrimitive};
use crate::codec::{Writer, Encodable, SizedField};
use crate::pdu::downlink::partial::{Offset, Timeslots};

pub(crate) enum HyperframeOrCipherKey {
    Hyperframe {
        hyperframe_number: u32
    },
    CipherKey {
        cck_id_or_key_version_number: u32
    }
}

impl Encodable for HyperframeOrCipherKey {
    fn encode(&self, writer: &mut Writer) {
        match self {
            HyperframeOrCipherKey::Hyperframe { hyperframe_number: hyperframe } => {
                writer.write_bool(false);
                writer.write_int(*hyperframe, 16);
            },
            HyperframeOrCipherKey::CipherKey { cck_id_or_key_version_number: cck_id } => {
                writer.write_bool(true);
                writer.write_int(*cck_id, 16);
            }
        }
    }
}

type TSModeBitmap = [bool; 20];

impl Encodable for TSModeBitmap {
    fn encode(&self, writer: &mut Writer) {
        for bit in self.iter() {
            writer.write_bool(*bit);
        }
    }
}

pub(crate) enum Immediate {
    AlwaysRandomise,
    AfterFrames(u32),
    Immediate
}

impl Encodable for Immediate {
    fn encode(&self, writer: &mut Writer) {
        match self {
            Immediate::AlwaysRandomise => writer.write_int(0b0000, 4),
            Immediate::Immediate => writer.write_int(0b1111, 4),
            Immediate::AfterFrames(after) => writer.write_int(*after, 4)
        }
    }
}

pub(crate) enum TimeslotPointer {
    SameAsDownlink,
    InTimeslots(Timeslots)
}

impl Encodable for TimeslotPointer {
    fn encode(&self, writer: &mut Writer) {
        match self {
            TimeslotPointer::SameAsDownlink => writer.write_int(0b0000, 4),
            TimeslotPointer::InTimeslots(timeslots) => timeslots.encode(writer)
        }
    }
}

pub(crate) struct AccessCodeDefinition {
    pub(crate) immediate: Immediate,
    pub(crate) waiting_time_opportunities: u32,
    pub(crate) number_of_attempts: u32,
    pub(crate) frame_length_x4: bool,
    pub(crate) timeslot: TimeslotPointer,
    pub(crate) minimum_priority: u32
}

impl Encodable for AccessCodeDefinition {
    fn encode(&self, writer: &mut Writer) {
        self.immediate.encode(writer);
        writer.write_int(self.waiting_time_opportunities, 4);
        writer.write_int(self.number_of_attempts, 4);
        writer.write_bool(self.frame_length_x4);
        self.timeslot.encode(writer);
        writer.write_int(self.minimum_priority, 3);
    }
}

pub(crate) struct ExtendedServicesBroadcast { }

impl Encodable for ExtendedServicesBroadcast {
    fn encode(&self, _writer: &mut Writer) {
        unimplemented!("extended services broadcast is not yet supported");
    }
}

pub enum OptionalField {
    TSModeEvenMultiframe(TSModeBitmap),
    TSModeOddMultiframe(TSModeBitmap),
    DefaultAccessCodeA(AccessCodeDefinition),
    ExtendedServicesBroadcast(ExtendedServicesBroadcast)
}

impl Encodable for OptionalField {
    fn encode(&self, writer: &mut Writer) {
        match self {
            OptionalField::TSModeEvenMultiframe(bitmap) => {
                writer.write_int(0b00, 2);
                bitmap.encode(writer);
            },
            OptionalField::TSModeOddMultiframe(bitmap) => {
                writer.write_int(0b01, 2);
                bitmap.encode(writer);
            },
            OptionalField::DefaultAccessCodeA(access_code_definition) => {
                writer.write_int(0b10, 2);
                access_code_definition.encode(writer);
            },
            OptionalField::ExtendedServicesBroadcast(extended_services_broadcast) => {
                writer.write_int(0b11, 2);
                extended_services_broadcast.encode(writer);
            }
        }
    }
}


#[derive(FromPrimitive, ToPrimitive)]
pub(crate) enum NumberOfCommonSCCH {
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

pub(crate) struct RFParameters {
    pub(crate) ms_txpwr_max_cell: u32,
    pub(crate) rxlev_access_min: u32,
    pub(crate) access_parameter: u32,
    pub(crate) radio_downlink_timeout: u32
}

impl Encodable for RFParameters {
    fn encode(&self, writer: &mut Writer) {
        writer.write_int(self.ms_txpwr_max_cell, 3);
        writer.write_int(self.rxlev_access_min, 4);
        writer.write_int(self.access_parameter, 4);
        writer.write_int(self.radio_downlink_timeout, 4);
    }
}

pub struct Sysinfo {
    pub main_carrier: u32,
    pub frequency_band: u32,
    pub offset: Offset,
    pub duplex_spacing: u32,
    pub reverse: bool,
    pub number_of_common_scch: NumberOfCommonSCCH,
    pub rf_parameters: RFParameters,
    pub hyperframe_or_cipher_key: HyperframeOrCipherKey,
    pub optional_field: OptionalField
}

impl Encodable for Sysinfo {
    fn encode(&self, writer: &mut Writer) {
        writer.write_int(self.main_carrier, 12);
        writer.write_int(self.frequency_band, 4);
        self.offset.encode(writer);
        writer.write_int(self.duplex_spacing, 3);
        writer.write_bool(self.reverse);
        self.number_of_common_scch.encode(writer);
        self.rf_parameters.encode(writer);
        self.hyperframe_or_cipher_key.encode(writer);
        self.optional_field.encode(writer);
    }
}

mod test {

    use super::*;

    #[test]
    fn encodes() {
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

        let mut writer = Writer::new();
        sysinfo.encode(&mut writer);
        let result = writer.done();

        dbg!(result);


    }

}