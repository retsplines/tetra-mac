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

struct Sysinfo {

    main_carrier: u32,
    frequency_band: u32,
    // offset: Offset,
    duplex_spacing: u32,
    // rf_parameters: RFParameters
}
