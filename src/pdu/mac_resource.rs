enum EncryptionMode {
    NotEncrypted = 0b00,
    EncryptedA = 0b01,
    EncryptedB = 0b10,
    EncryptedC = 0b11
}

enum RandomAccessFlag {
    Undefined = 0b00
}

pub enum LengthIndication {
    Reserved,
    NullPDU,
    Octets(usize),
    SecondHalfSlotStolen,
    StartOfFragmentation
}

enum Address {
    NullPDU,
    SSI { address: u32 },
    EventLabel { event_label: u32 },
    USSI { ussi: u32 },
    SMI { smi: u32 },
    SSIPlusEventLabel { ssi: u32, event_label: u32 },
    SMIPlusUsageMarker { smi: u32, usage_marker: u32 },
    SMIPlusEventLabel { smi: u32, event_label: u32 }
}

enum PowerControl {
    NoChange,
    IncreaseBySteps(u32),
    MaximumPathDelayExceeded,
    OpenLoop,
    DecreaseBySteps(u32),
    RadioUplinkFailure
}

enum CapacityAllocation {
    FirstSubslot,
    Slots(u32),
    SecondSubslot
}

enum GrantingDelay {
    AtNextOpportunity,
    After(u32),
    Frame18,
    WaitForAnotherMessage
}

struct SlotGranting {
    capacity_allocation: CapacityAllocation,
    granting_delay: GrantingDelay 
}

enum AllocationType {
    Replacement,
    Addition,
    QuitAndGoTo,
    ReplacePlus
}

enum Direction {
    Downlink,
    Uplink,
    Both
}

enum TimeslotAssigned {
    AppropriateCCH,
    TimeslotAssigned(bool, bool, bool, bool)
}

enum MonitoringPatterns {
    None,
    One,
    Two,
    Three
}

struct ExtendedCarrierNumbering {
    frequency_band: u32,
    offset: u32,
    duplex_spacing: u32,
}

struct ChannelAllocation {
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

struct MACResourcePDU<'a> {
    fill_bit_indication: bool,
    grant_is_on_current_channel: bool,
    length: LengthIndication,
    address: Address,
    power_control: Option<PowerControl>,
    slot_granting: Option<SlotGranting>,
    channel_allocation: Option<ChannelAllocation>,
    tm_sdu: Option<&'a [u8]>
}
