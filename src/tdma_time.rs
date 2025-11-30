#[derive(Debug)]
pub struct TDMATime {
    slot: u32,

    // 4 slots
    frame: u32,

    // 18 frames
    multiframe: u32,

    // 60 multiframes
    hyperframe: u32
}

/// Representation of the system
impl TDMATime {

    fn as_slot_number(&self) -> u32 {
        (self.hyperframe * 60 * 18 * 4) + (self.multiframe * 18 * 4) + (self.frame * 4) + self.slot
    }

    /// Initialises a new TDMA timestamp with all counts set to 0
    pub fn new() -> Self {
        TDMATime {
            slot: 0,
            frame: 0,
            multiframe: 0,
            hyperframe: 0
        }
    }

    /// Returns the 1-based slot number, between 1 and 4
    pub fn slot(&self) -> u32 {
        self.slot + 1
    }

    /// Returns the 1-based frame number, between 1 and 18
    pub fn frame(&self) -> u32 {
        self.frame + 1
    }

    /// Returns the 1-based mulitframe number, between 1 and 60
    pub fn multiframe(&self) -> u32 {
        self.multiframe + 1
    }

    /// Returns the 1-based hyperframe number, between 1 and 65535
    pub fn hyperframe(&self) -> u32 {
        self.hyperframe + 1
    }

    /// Returns whether the current frame is the Control Frame (frame 18)
    pub fn is_control_frame(&self) -> bool {
        self.frame == 17
    }

    fn from_slot_number(slot_number: u32) -> Self {

        let hyperframe = (slot_number / (60 * 18 * 4)) % 65535;
        let slot_number = slot_number % (60 * 18 * 4);
        let multiframe = slot_number / (18 * 4);
        let slot_number = slot_number % (18 * 4);
        let frame = slot_number / 4;
        let slot = slot_number % 4;

        TDMATime {
            slot,
            frame,
            multiframe,
            hyperframe
        }
    }

    fn next(self) -> Self {
        let slot_number = self.as_slot_number();
        let next_slot_number = slot_number + 1;
        TDMATime::from_slot_number(next_slot_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot_number() {
        let tdma = TDMATime {
            slot: 3,
            frame: 2,
            multiframe: 1,
            hyperframe: 0
        };

        assert_eq!(
            tdma.as_slot_number(),
            3 + (2 * 4) + (18 * 4)
        );
    }

    #[test]
    fn test_from_slot_number() {
        let slot_number = 3 + (2 * 4) + (18 * 4);
        let tdma = TDMATime::from_slot_number(slot_number);

        assert_eq!(tdma.slot, 3);
        assert_eq!(tdma.frame, 2);
        assert_eq!(tdma.multiframe, 1);
        assert_eq!(tdma.hyperframe, 0);
    }

    #[test]
    fn test_next() {
        let tdma = TDMATime {
            slot: 3,
            frame: 2,
            multiframe: 1,
            hyperframe: 0
        };

        let next = tdma.next();

        assert_eq!(next.slot, 0);
        assert_eq!(next.frame, 3);
        assert_eq!(next.multiframe, 1);
        assert_eq!(next.hyperframe, 0);
    }

    #[test]
    fn test_next_wrap() {
        let tdma = TDMATime {
            slot: 3, // next slot is 0
            frame: 17, // next frame is 0
            multiframe: 59, // next multiframe is 0
            hyperframe: 0 // next hyperframe is 1
        };

        let next = tdma.next();

        assert_eq!(next.slot, 0);
        assert_eq!(next.frame, 0);
        assert_eq!(next.multiframe, 0);
        assert_eq!(next.hyperframe, 1);
    }
}