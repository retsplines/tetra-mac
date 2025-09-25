use std::fmt::{Display, Formatter};
use bitvec::prelude::*;
use crate::bits::Bits;

#[derive(Clone)]
pub struct State {
    pub state: Bits
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for bit in self.state.iter() {
            write!(f, "{} ", if *bit { 1 } else { 0 })?;
        }
        Ok(())
    }
}

impl State {

    pub fn new(mcc: u32, mnc: u32, colour: u32) -> State {

        // The first two bits are set
        let state = bits![mut u8, Msb0; 0; 32];
        state[0..10].store_be(mcc);
        state[10..24].store_be(mnc);
        state[24..30].store_be(colour);

        // Plus two padding bits, k=-31, k=-30
        state[30..32].store_be(0b11);

        State { state: Bits::from_bitslice(state) }
    }

    pub fn shift(&mut self, bit: bool) {

        // Remove the last bit
        self.state.drain(self.state.len() - 1..self.state.len());

        // Insert the new bit at the start (left-most position)
        self.state.insert(0, bit);

    }
}

fn taps() -> Vec<u8> {
    vec![32, 26, 23, 22, 16, 12, 11, 10, 8, 7, 5, 4, 2, 1]
}

/// Generate a single bit from the LFSR
/// Updates the LFSR state and returns the new bit
fn lfsr_bit(state: &mut State) -> bool {

    let mut bit = 0;

    for tap in taps() {
        bit ^= state.state[(tap as usize) - 1] as u32;
    }

    // Get the LSB
    bit &= 1;

    // Shift the bit into the LFSR state
    state.shift(bit != 0);

    bit != 0
}

pub fn scrambler_encode(block: &Bits, initial_state: &State) -> Bits {

    // Clone the initial state
    let mut scrambler_state = initial_state.clone();

    // Clone the input block
    let mut scrambled = block.clone();

    // For each bit in block, xor with a LFSR bit
    for (index, bit) in block.iter().enumerate() {
        let lfsr = lfsr_bit(&mut scrambler_state);
        let res = *bit ^ lfsr;
        scrambled.set(index, res);
    }

    scrambled
}

pub fn scrambler_decode(block: &Bits, initial_state: &State) -> Bits {

    // Same as encoding
    scrambler_encode(block, initial_state)

}

#[cfg(test)]
mod test {

    use bitvec::prelude::*;
    use crate::new_bits;

    #[test]
    #[ignore]
    /// todo: fix these tests, they assume wrong bit order
    fn state_shifts_correctly() {

        // Start with all 1s
        let mut scrambler_state = super::State::new(0xffff, 0xffff, 0xff);
        assert_eq!(scrambler_state.state.load_be::<u32>(), 0xffffffff);

        // Shift in a 0
        assert!(scrambler_state.state[31]);
        scrambler_state.shift(false);
        assert!(!scrambler_state.state[31]);

        // Now shift in a 1
        scrambler_state.shift(true);
        assert!(scrambler_state.state[31]);

        // Check the bit positions in the state
        // (This is actually how the BSCH is scrambled)
        scrambler_state = super::State::new(0, 0, 0);
        assert_eq!(scrambler_state.state.load_be::<u32>(), 0x00000003);

        // Colour
        scrambler_state = super::State::new(0, 0, 0xff);
        assert_eq!(scrambler_state.state.load_be::<u32>(), 0x000000ff);

        // MCC
        scrambler_state = super::State::new(0xffff, 0, 0);
        assert_eq!(scrambler_state.state.load_be::<u32>(), 0xffc00003);

        // MNC
        scrambler_state = super::State::new(0, 0xffff, 0);
        assert_eq!(scrambler_state.state.load_be::<u32>(), 0x003fff03);

        // A specific MCC/MNC/Colour
        scrambler_state = super::State::new(234, 30, 17);
        assert_eq!(scrambler_state.state.load_be::<u32>(), 0x3a801e47);
    }

    #[test]
    fn scrambles_correctly() {

        // Use the BSCH scrambling code
        let mut scrambler_state = super::State::new(0, 0, 0);

        // Simple test
        let block = new_bits![0, 1, 0, 1];
        let scrambled = super::scrambler_encode(&block, &scrambler_state);

        // Reset the scrambler state
        scrambler_state = super::State::new(0, 0, 0);

        // Verify that descrambling gets us back to the original
        let descrambled = super::scrambler_decode(&scrambled, &scrambler_state);

        assert_eq!(descrambled, block);
    }

}