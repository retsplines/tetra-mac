use crate::pdu::downlink::partial::Offset;

enum HyperframeOrCipherKey {
    Hyperframe {
        hyperframe_number: u32
    },
    CipherKey {
        cck_id_or_key_version_number: u32
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
    minimum_priority: Priority
}

enum OptionalField {
    TSModeEvenMultiframe (TSModeBitmap),
    TSModeOddMultiframe (TSModeBitmap),
    DefaultAccessCodeA,
    ExtendedServicesBroadcast
}

enum NumberOfCommonSCCH {
    None = 0b00,
    Timeslot2 = 0b01,
    Timeslot23 = 0b10,
    Timeslot234 = 0b11
}

struct RFParameters {
    ms_txpwr_max_cell: u32,
    rxlev_access_min: u32,
    access_parameter: u32,
    radio_downlink_timeout: u32
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
