use bitvec::prelude::BitVec;
use crate::lower::rcpc::state::State;
use crate::lower::rcpc::puncturers::{Puncturer};

macro_rules! bit {
    ($b:expr) => {
        $b as u8
    };
}

/// Encode a bit into a 4-bit codeword
pub fn encode_bit(bit: bool, state: &mut State) -> [bool; 4] {

    let mut output: [bool; 4] =  [false; 4];

    // Compute the codeword based on the generator polynomials
    // G₁(D) = 1 + D + D⁴; G₁(2) = 19
    // G₂(D) = 1 + D² + D³ + D⁴; G₂(2) = 29
    // G₃(D) = 1 + D + D² + D⁴; G₃(2) = 23
    // G₄(D) = 1 + D + D³ + D⁴; G₄(2) = 27
    output[0] = (bit!(bit) + bit!(state[0]) + bit!(state[3])) % 2 > 0;
    output[1] = (bit!(bit) + bit!(state[1]) + bit!(state[2]) + bit!(state[3])) % 2 > 0;
    output[2] = (bit!(bit) + bit!(state[0]) + bit!(state[1]) + bit!(state[3])) % 2 > 0;
    output[3] = (bit!(bit) + bit!(state[0]) + bit!(state[2]) + bit!(state[3])) % 2 > 0;

    // Shift the bit in
    state.shift_in(bit);

    output
}

fn puncture(mother: &BitVec, puncturer: &Puncturer) -> BitVec {

    let base = mother.len() / 4;

    // Is puncturing possible?
    if base as i32 * puncturer.denominator % puncturer.numerator != 0 {
        // Doesn't exactly divide, so puncturing not possible
        panic!(
            "not possible to puncture code of length {} with puncturer {}/{}",
            mother.len(), puncturer.numerator, puncturer.denominator
        );
    }

    // Start with all-1s
    let punctured_len = (base * puncturer.denominator as usize) / puncturer.numerator as usize;
    let mut punctured = BitVec::repeat(true, punctured_len);

    // Output bit at x should be set to i(x)
    for out_index in 1..punctured_len + 1 {

        let i = (puncturer.i)(out_index);
        let k = puncturer.period * ((i - 1) / puncturer.t) + puncturer.coefficients[
            i - puncturer.t * ((i - 1) / puncturer.t)
        ];
        punctured.set(out_index - 1, mother[k - 1])

    }

    punctured
}

struct Depunctured {
    mother: BitVec,
    valid_mask: BitVec
}

/// Depuncture a punctured code
fn depuncture(punctured: &BitVec, puncturer: &Puncturer) -> Depunctured {

    let depunctured_len =
        ((punctured.len() * puncturer.numerator as usize) / puncturer.denominator as usize) * 4;

    let mut mother = BitVec::repeat(false, depunctured_len);
    let mut valid_mask = BitVec::repeat(false, depunctured_len);

    for in_index in 1..punctured.len() + 1 {
        let i = (puncturer.i)(in_index);
        let k = puncturer.period * ((i - 1) / puncturer.t) + puncturer.coefficients[
            i - puncturer.t * ((i - 1) / puncturer.t)
        ];
        mother.set(k - 1, punctured[in_index - 1]);
        valid_mask.set(k - 1, true);
    }

    Depunctured {
        mother,
        valid_mask,
    }
}

/// RCPC-encode a block using the specified puncturer
fn encode(block: &BitVec, maybe_puncturer: Option<&Puncturer>, state: &mut State) -> BitVec {

    let mut encoded = BitVec::new();

    // Generate the mother code
    for in_bit in block.iter() {
        encoded.extend(encode_bit(*in_bit, state).iter())
    }

    if let Some(puncturer) = maybe_puncturer {
        return puncture(&encoded, puncturer);
    }

    // No puncturing required
    encoded
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::*;
    use crate::lower::rcpc::puncturers::PredefinedPuncturer::{Rate1Over3Puncturer, Rate2Over3Puncturer};
    use super::*;

    /// Check that the mother code behaviour is consistent with osmo-tetra
    #[test]
    fn mother_code_consistent_with_osmo_tetra() {
        let orig = bitvec![0, 1, 0, 1, 0, 1, 0, 1];
        let mut state = State::new();
        let mother = encode(&orig, None, &mut state);
        let expt_mother = bitvec![0,0,0,0,1,1,1,1,1,0,1,1,1,0,0,1,1,1,1,0,0,1,1,0,1,1,1,0,0,1,1,0];
        assert_eq!(mother.as_bitslice(), expt_mother.as_bitslice());
    }

    #[test]
    fn punctures_correctly() {

        let puncturers_to_test = [
            Rate1Over3Puncturer,
            Rate2Over3Puncturer,
        ];

        // Generate a 32-bit alternating 1010... pattern
        let pattern = bits![mut 0; 32];
        pattern.fill_with(|idx| idx % 2 == 0);

        for puncturer_to_test in puncturers_to_test.iter() {

            // Puncture
            let mother_code = BitVec::from_bitslice(pattern);
            let puncturer = Puncturer::build(puncturer_to_test);

            println!("bit pattern {} len {}", mother_code, mother_code.len());
            let punctured = puncture(&mother_code, &puncturer);

            println!("Mother code {:?}, punctured to {:?}", mother_code, punctured);

            // Depuncture and compare
            let depunctured = depuncture(&punctured, &puncturer);

            // Compare only the valid bits
            for valid_index in depunctured.valid_mask.iter_ones() {
                assert_eq!(depunctured.mother[valid_index], mother_code[valid_index]);
            }
        }
    }

}