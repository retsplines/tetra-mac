use bitvec::macros::internal::funty::Fundamental;
use bitvec::prelude::*;
use crate::bits::Bits;

#[derive(Clone)]
pub struct State {
    pub state: BitVec
}

impl State {

    pub fn new(mcc: u32, mnc: u32, colour: u32) -> State {

        let mut state = BitVec::new();

        // The first two bits are set
        state.extend_from_bitslice(bitvec![1, 1].as_bitslice());

        // Copy the 6-bit colour, 14-bit MNC and 10-bit MCC into the state
        state.extend_from_bitslice(&colour.view_bits::<Lsb0>()[0..6]);
        state.extend_from_bitslice(&mnc.view_bits::<Lsb0>()[0..14]);
        state.extend_from_bitslice(&mcc.view_bits::<Lsb0>()[0..10]);

        println!("Initial state: {:?}", state.to_string());

        State { state }
    }

    pub fn shift(&mut self, bit: bool) {

        // Drop the first bit
        self.state.drain(0..1);

        // Shift the new bit into the start of state
        self.state.push(bit);

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

pub fn scrambler_encode(block: &Bits, scrambler_state: &mut State) -> Bits {

    // Clone the input block
    let mut scrambled = block.clone();

    // For each bit in block, xor with a LFSR bit
    for (index, bit) in block.iter().enumerate() {
        scrambled.set(index, bit.as_bool() ^ lfsr_bit(scrambler_state));
    }

    scrambled
}

pub fn scrambler_decode(block: &Bits, state: &mut State) -> Bits {

    // Same as encoding
    scrambler_encode(block, state)

}

#[cfg(test)]
mod test {

    use bitvec::prelude::*;
    use crate::new_bits;

    #[test]
    fn state_shifts_correctly() {

        // Start with all 1s
        let mut scrambler_state = super::State::new(0xffff, 0xffff, 0xff);
        assert_eq!(scrambler_state.state.load::<u32>(), 0xffffffff);

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
        println!("Scram state: {:?}", scrambler_state.state.to_string());
        assert_eq!(scrambler_state.state.load::<u32>(), 0x00000003);

        // MCC
        scrambler_state = super::State::new(0xffff, 0, 0);
        assert_eq!(scrambler_state.state.load::<u32>(), 0xffc00003);

        // MNC
        scrambler_state = super::State::new(0, 0xffff, 0);
        assert_eq!(scrambler_state.state.load::<u32>(), 0x003fff03);

        // Colour
        scrambler_state = super::State::new(0, 0, 0xff);
        assert_eq!(scrambler_state.state.load::<u32>(), 0x000000ff);

        // A specific MCC/MNC/Colour
        scrambler_state = super::State::new(234, 30, 17);
        assert_eq!(scrambler_state.state.load::<u32>(), 0x3a801e47);
    }

    #[test]
    fn scrambles_correctly() {

        // Use the BSCH scrambling code
        let mut scrambler_state = super::State::new(0, 0, 0);

        // Simple test
        let block = new_bits![0, 1, 0, 1];
        let scrambled = super::scrambler_encode(&block, &mut scrambler_state);

        // Reset the scrambler state
        scrambler_state = super::State::new(0, 0, 0);

        // Verify that descrambling gets us back to the original
        let descrambled = super::scrambler_decode(&scrambled, &mut scrambler_state);

        assert_eq!(descrambled, block);
    }

}