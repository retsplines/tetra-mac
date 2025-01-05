use bitvec::prelude::*;
use super::{cursor::Cursor, encoder::Decodable};

#[derive(Debug)]
enum EncryptionMode {
    NotEncrypted = 0b00,
    EncryptedA = 0b01,
    EncryptedB = 0b10,
    EncryptedC = 0b11
}

#[derive(Debug)]
enum RandomAccessFlag {
    Undefined = 0b00
}

#[derive(Debug)]
enum LengthIndication {
    Reserved,
    NullPDU,
    Octets(usize),
    SecondHalfSlotStolen,
    StartOfFragmentation
}

impl Decodable for LengthIndication {
    fn decode(cursor: &mut Cursor) -> Self {
        let length_field = cursor.read_int(6);
        match length_field {
            0b000000 | 0b000001 => Self::Reserved,
            0b000010 => Self::NullPDU,
            0b000011 => Self::Reserved,
            0b100011..=0b111101 => Self:: Reserved,
            0b111110 => Self::SecondHalfSlotStolen,
            0b111111 => Self::StartOfFragmentation,
            octets => Self::Octets(octets as usize),
        }
    }
}

#[derive(Debug)]
enum Address {
    NullPDU,
    SSI { address: u32 },
    EventLabel { event_label: u32 },
    USSI { ussi: u32 },
    SMI { smi: u32 },
    SSIPlusEventLabel { ssi: u32, event_label: u32 },
    SSIPlusUsageMarker { ssi: u32, usage_marker: u32 },
    SMIPlusEventLabel { smi: u32, event_label: u32 }
}

impl Decodable for Address {
    fn decode(cursor: &mut Cursor) -> Self {
        let address_type_field = cursor.read_int(3);
        match address_type_field {
            0b000 => Address::NullPDU,
            0b001 => Address::SSI { address: cursor.read_int(24) },
            0b010 => Address::EventLabel { event_label: cursor.read_int(10) },
            0b011 => Address::USSI { ussi: cursor.read_int(24) },
            0b100 => Address::SMI { smi: cursor.read_int(24) },
            0b101 => Address::SSIPlusEventLabel { 
                ssi: cursor.read_int(24),
                event_label: cursor.read_int(10)
            },
            0b110 => Address::SSIPlusUsageMarker {
                ssi: cursor.read_int(24),
                usage_marker: cursor.read_int(10)
            },
            0b111 => Address::SMIPlusEventLabel {
                smi: cursor.read_int(24),
                event_label: cursor.read_int(10)
            },
            unknown => panic!("unknown address type {unknown}")
        }
    }
}

#[derive(Debug)]
enum PowerControl {
    NoChange,
    IncreaseBySteps(u32),
    MaximumPathDelayExceeded,
    OpenLoop,
    DecreaseBySteps(u32),
    RadioUplinkFailure
}

type PowerControlOptional = Option<PowerControl>;

impl Decodable for PowerControlOptional {
    fn decode(cursor: &mut Cursor) -> Self {
        Some(match cursor.read_int_optional(4)? {
            0b0000 => PowerControl::NoChange,
            inc @ 0b0001 ..= 0b0110 => PowerControl::IncreaseBySteps(inc),
            0b0111 => PowerControl::MaximumPathDelayExceeded,
            0b1000 => PowerControl::OpenLoop,
            dec @ 0b1001 ..= 0b1110 => PowerControl::DecreaseBySteps(dec - 8),
            0b1111 => PowerControl::RadioUplinkFailure,
            unknown @ _ => panic!("unknown power control information {unknown}")
        })
    }
}

#[derive(Debug)]
enum CapacityAllocation {
    FirstSubslot,
    Slots(u32),
    SecondSubslot
}

#[derive(Debug)]
enum GrantingDelay {
    AtNextOpportunity,
    After(u32),
    Frame18,
    WaitForAnotherMessage
}

#[derive(Debug)]
struct SlotGranting {
    capacity_allocation: CapacityAllocation,
    granting_delay: GrantingDelay 
}

#[derive(Debug)]
enum AllocationType {
    Replacement,
    Addition,
    QuitAndGoTo,
    ReplacePlus
}

#[derive(Debug)]
enum Direction {
    Downlink,
    Uplink,
    Both
}

#[derive(Debug)]
enum TimeslotAssigned {
    AppropriateCCH,
    TimeslotAssigned(bool, bool, bool, bool)
}

#[derive(Debug)]
enum MonitoringPatterns {
    None,
    One,
    Two,
    Three
}

#[derive(Debug)]
struct ExtendedCarrierNumbering {
    frequency_band: u32,
    offset: u32,
    duplex_spacing: u32,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct MACResourcePDU {
    pub fill_bit_indication: bool,
    pub grant_is_on_current_channel: bool,
    pub length: LengthIndication,
    address: Address,
    power_control: Option<PowerControl>,
    slot_granting: Option<SlotGranting>,
    channel_allocation: Option<ChannelAllocation>,
    tm_sdu: Option<Vec<u8>>
}

impl Decodable for MACResourcePDU {
    fn decode(cursor: &mut Cursor) -> Self {
        
        // Decode the fill bit indication
        let fill_bit_indication = cursor.read_bool();

        // Decode the grant information
        let grant_is_on_current_channel = cursor.read_bool();

        // Length information
        let length = LengthIndication::decode(cursor);

        let address = Address::decode(cursor);

        let power_control = PowerControlOptional::decode(cursor);



        Self {
            fill_bit_indication,
            grant_is_on_current_channel,
            length,
            address,
            power_control: power_control,
            slot_granting: None,
            channel_allocation: None,
            tm_sdu: Some(vec![1_u8])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let data: Vec<u8> = vec![
            0x20, 0x69, 0x00, 0x04, 0x02, 0x03, 0x48, 0x40,
            0x00, 0x00, 0x4e, 0xab, 0x10, 0x00, 0x10, 0x80
        ];

        // Create a cursor over the data
        let mut cur = Cursor::new(data.as_bits());

        let pdu = MACResourcePDU::decode(&mut cur);
        println!("{:?}", pdu);

    }
}