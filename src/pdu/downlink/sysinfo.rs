use num_derive::{FromPrimitive, ToPrimitive};
use crate::bits::Bits;
use crate::codec::{Writer, Encodable, SizedField, Decodable, Reader};
use crate::pdu::downlink::partial::{Offset, Timeslots};
use crate::pdu::{BroadcastPDUType, DownlinkMACPDUType};

#[derive(Debug)]
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

impl Decodable for HyperframeOrCipherKey {
    fn decode(reader: &mut Reader) -> Self {
        if reader.read_bool() {
            HyperframeOrCipherKey::CipherKey {
                cck_id_or_key_version_number: reader.read_int(16)
            }
        } else {
            HyperframeOrCipherKey::Hyperframe {
                hyperframe_number: reader.read_int(16)
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

#[derive(Debug)]
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

impl Decodable for Immediate {
    fn decode(reader: &mut Reader) -> Self {
        match reader.read_int(4) {
            0b0000 => Immediate::AlwaysRandomise,
            0b1111 => Immediate::Immediate,
            n => Immediate::AfterFrames(n),
        }
    }
}

#[derive(Debug)]
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

impl Decodable for TimeslotPointer {
    fn decode(reader: &mut Reader) -> Self {
        match reader.read_int(4) {
            0b0000 => TimeslotPointer::SameAsDownlink,
            _ => TimeslotPointer::InTimeslots(Timeslots::decode(reader)),
        }
    }
}

#[derive(Debug)]
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

#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum SDSTLAddressingMethod {
    Reserved = 0b00,
    ServiceCentreAddressingPreferred = 0b01,
    NeverUseServiceCentreAddressing = 0b10,
    MSChoiceToUseServiceCentreAddressing = 0b11
}

impl SizedField for SDSTLAddressingMethod {
    fn size() -> usize {
        2
    }
}

#[derive(Debug)]
pub(crate) struct ExtendedServicesBroadcast {
    // todo: this is a whole separate structure described in EN 300 392-7
    security_information: u32,
    sds_tl_addressing_method: SDSTLAddressingMethod
}

impl Encodable for ExtendedServicesBroadcast {
    fn encode(&self, writer: &mut Writer) {
        writer.write_int(self.security_information, 8);
        self.sds_tl_addressing_method.encode(writer);
        writer.write_int(0, 10); // reserved
    }
}

impl Decodable for ExtendedServicesBroadcast {
    fn decode(reader: &mut Reader) -> Self {

        let result = ExtendedServicesBroadcast {
            security_information: reader.read_int(8),
            sds_tl_addressing_method: SDSTLAddressingMethod::decode(reader),
        };

        // reserved, set to all-0
        assert_eq!(reader.read_int(10), 0);

        result
    }
}

#[derive(Debug)]
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

impl Decodable for OptionalField {
    fn decode(reader: &mut Reader) -> Self {
        match reader.read_int(2) {
            0b00 => OptionalField::TSModeEvenMultiframe({
                let mut bitmap = [false; 20];
                for bit in bitmap.iter_mut() {
                    *bit = reader.read_bool();
                }
                bitmap
            }),
            0b01 => OptionalField::TSModeOddMultiframe({
                let mut bitmap = [false; 20];
                for bit in bitmap.iter_mut() {
                    *bit = reader.read_bool();
                }
                bitmap
            }),
            0b10 => OptionalField::DefaultAccessCodeA(AccessCodeDefinition {
                immediate: Immediate::decode(reader),
                waiting_time_opportunities: reader.read_int(4),
                number_of_attempts: reader.read_int(4),
                frame_length_x4: reader.read_bool(),
                timeslot: TimeslotPointer::decode(reader),
                minimum_priority: reader.read_int(3),
            }),
            0b11 => OptionalField::ExtendedServicesBroadcast(
                ExtendedServicesBroadcast::decode(reader)
            ),
            _ => panic!("invalid OptionalField discriminator"),
        }
    }
}


#[derive(FromPrimitive, ToPrimitive, Debug)]
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

#[derive(Debug)]
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

impl Decodable for RFParameters {
    fn decode(reader: &mut Reader) -> Self {
        RFParameters {
            ms_txpwr_max_cell: reader.read_int(3),
            rxlev_access_min: reader.read_int(4),
            access_parameter: reader.read_int(4),
            radio_downlink_timeout: reader.read_int(4),
        }
    }
}

#[derive(Debug)]
pub struct Sysinfo {
    pub main_carrier: u32,
    pub frequency_band: u32,
    pub offset: Offset,
    pub duplex_spacing: u32,
    pub reverse: bool,
    pub number_of_common_scch: NumberOfCommonSCCH,
    pub rf_parameters: RFParameters,
    pub hyperframe_or_cipher_key: HyperframeOrCipherKey,
    pub optional_field: OptionalField,
    pub tm_sdu_bits: Bits
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

impl Decodable for Sysinfo {
    fn decode(reader: &mut Reader) -> Self {

        // Decode & validate the PDU type
        let pdu_type = DownlinkMACPDUType::decode(reader);
        assert_eq!(pdu_type, DownlinkMACPDUType::Broadcast);

        let broadcast_type = BroadcastPDUType::decode(reader);
        assert_eq!(broadcast_type, BroadcastPDUType::Sysinfo);

        let result = Sysinfo {
            main_carrier: reader.read_int(12),
            frequency_band: reader.read_int(4),
            offset: Offset::decode(reader),
            duplex_spacing: reader.read_int(3),
            reverse: reader.read_bool(),
            number_of_common_scch: NumberOfCommonSCCH::decode(reader),
            rf_parameters: RFParameters::decode(reader),
            hyperframe_or_cipher_key: HyperframeOrCipherKey::decode(reader),
            optional_field: OptionalField::decode(reader),
            tm_sdu_bits: Bits::new()
        };

        let tm_sdu_bits = reader.read(42);

        Sysinfo {
            tm_sdu_bits,
            ..result
        }
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
            }),
            tm_sdu_bits: Bits::repeat(false, 42)
        };

        let mut writer = Writer::new();
        sysinfo.encode(&mut writer);
    }

}