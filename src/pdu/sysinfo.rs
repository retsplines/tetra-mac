enum Offset {
    NoOffset = 0b00,
    Plus6_25kHz = 0b01,
    Minus6_25kHz = 0b10,
    Plus12_5kHz = 0b11
}

enum NumberOfCommonSCCH {
    None = 0b00,
    Timeslot2 = 0b01,
    Timeslot23 = 0b10,
    Timeslot234 = 0b11
}

struct RFParameters {
    MsTxpwrMaxCell: u32,
    RxlevAccessMin: u32,
    AccessParameter: u32,
    RadioDownlinkTimeout: u32
}

enum HyperframeOrCipherKeyFlag {
    Hyperframe {
        hyperframe_number: u32
    },
    CipherKey {
        cck_id_or_key_version_number: u32
    }
}

type TSModeBitmap = [bool; 20];

enum OptionalField {
    TSModeEvenMultiframe (TSModeBitmap),
    TSModeOddMultiframe (TSModeBitmap),
    DefaultAccessCodeA,
    ExtendedServicesBroadcast
}

struct MACSysinfoPDU {

    main_carrier: u32,
    frequency_band: u32,
    offset: Offset,
    duplex_spacing: u32,
    rf_parameters: RFParameters
}
