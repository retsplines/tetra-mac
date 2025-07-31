use crate::lower::rcpc::state::State;

/// Encode a bit into a codeword
fn encode_bit(bit: bool, mut state: State) -> [bool; 4] {

    let mut output: [bool; 4] =  [false; 4];

    // Compute the codeword based on the generator polynomials
    // G₁(D) = 1 + D + D⁴; G₁(2) = 19
    // G₂(D) = 1 + D² + D³ + D⁴; G₂(2) = 29
    // G₃(D) = 1 + D + D² + D⁴; G₃(2) = 23
    // G₄(D) = 1 + D + D³ + D⁴; G₄(2) = 27
    output[0] = (bit as u8 + state[0] as u8 + state[3] as u8 % 2);
}