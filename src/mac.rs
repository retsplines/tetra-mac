use crate::Bits;
use crate::tdma_time::TDMATime;

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

    fn slot_should_be_sync(&self) -> bool {

        // During the control frame, sync slot is in TN 4-(MN+1)%4
        if self.tdma.is_control_frame() {
            return (4 - (self.tdma.multiframe() + 1) % 4) == self.tdma.slot();
        }

        // Otherwise, sync in slot 3
        self.tdma.slot() == 3
    }

    fn slot_should_be_bnch(&self) -> bool {
        (4 - (self.tdma.multiframe() + 3) % 4) == self.tdma.slot()
    }

    pub fn generate_next_slot(&self) {

        // MAC PDUs that we'll prep for this slot
        // Multiple PDUs may be generated and sent together in a slot ("association")
        let mut pdus: Vec<Bits> = Vec::new();

        // Decide what to generate based on the usage of this slot
        match self.downlink_usage[self.tdma.slot() as usize] {
            DownlinkPhysicalUsage::Control => {}

            // Do nothing with other slot types for now
            _ => {}
        }


    }
}

