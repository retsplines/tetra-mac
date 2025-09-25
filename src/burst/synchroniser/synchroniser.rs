use crate::bits::Bits;

enum State {

    // Haven't seen any training sequence yet
    Unlocked,

    // Have seen a training sequence, but not currently receiving a burst
    Locked,

    // Have seen a training sequence and currently receiving a burst
    LockedAndBurstStarted,
}

/// Which type of bursts the synchroniser should search for
enum SyncMode {
    Uplink,
    Downlink
}

struct Synchroniser {
    state: State,
    mode: SyncMode,
    buffer: Bits
}

/// TETRA burst synchroniser
///
/// A chunk of bits is passed in.
/// If we don't have enough space in the rolling buffer, age-out enough bits to make space.
/// Copy the bits into the end of the rolling buffer.
impl Synchroniser {

    pub fn new() -> Synchroniser {
        Synchroniser {
            state: State::Unlocked,
            mode: SyncMode::Uplink,
            buffer: Bits::new()
        }
    }

    fn append_bits_to_buffer(&mut self, bits: Bits) {

        // Check if there's enough space
        let space_left = self.buffer.capacity() - self.buffer.len();
        if space_left < bits.len() {
            // Shift out enough bits to fit the new ones in
            let space_needed = bits.len() - space_left;
            self.buffer.resize(space_needed, false);
        }

    }

    pub fn feed(&self, bits: Bits) {

    }

}