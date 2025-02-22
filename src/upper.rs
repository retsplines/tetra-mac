use crate::Bits;
use crate::tdma_time::TDMATime;
use crate::pdu::downlink::MACResourcePDU;

enum DownlinkPhysicalUsage {
    Control,
    Traffic,
    Unallocated
}

struct MAC {
    tdma: TDMATime,
    downlink_usage: [DownlinkPhysicalUsage; 4]
}

impl MAC {
    pub fn new() -> Self {
        MAC {

            tdma: TDMATime::new(),
            downlink_usage: [
                DownlinkPhysicalUsage::Control,
                DownlinkPhysicalUsage::Unallocated,
                DownlinkPhysicalUsage::Unallocated,
                DownlinkPhysicalUsage::Unallocated
            ]
        }
    }

    fn slot_should_be_bnch(&self) -> bool {
        // During the control frame, BNCH appears if (MN + TN) % 4 == 1
        self.tdma.is_control_frame() && (self.tdma.multiframe() + self.tdma.slot()) % 4 == 1
    }

    fn slot_should_be_bsch(&self) -> bool {
        // During the control frame, BSCH appears if (MN + TN) % 4 = 3
        self.tdma.is_control_frame() && (self.tdma.multiframe() + self.tdma.slot()) % 4 == 3
    }

    fn generate_null_schf(&self) -> Bits {

        // Create a NULL MAC-RESOURCE
        let mac_resource = MACResourcePDU {
            fill_bit_indication: false,
            grant_is_on_current_channel: false,
            encryption_mode: 0,
            random_access_acknowledged: false,
            length: Length::Reserved,
            address: Address::NullPDU,
            power_control: Optional::Absent,
            slot_granting: Optional::Absent,
            channel_allocation: Optional::Absent,
        };
        
        todo!()
    }

    /// Generate the next downlink slot
    pub fn generate_next_slot(&self) {

        // MAC PDUs that we'll prep for this slot
        // Multiple PDUs may be generated and sent together in a slot ("association")
        let mut pdus: Vec<Bits> = Vec::new();
        
        // Firstly, decide what the downlink usage of this slot is

        // Decide what to generate based on the usage of this slot
        match self.downlink_usage[self.tdma.slot() as usize] {

            // Unallocated, map SCH/F containing a NULL PDU
            DownlinkPhysicalUsage::Unallocated => {},

            DownlinkPhysicalUsage::Control => {}

            // Do nothing with other slot types for now
            _ => {}
        }


    }
}

