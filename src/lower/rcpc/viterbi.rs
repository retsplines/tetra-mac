use crate::bits::Bits;
use crate::lower::rcpc::coder::encode_bit;
use crate::lower::rcpc::state::State;

#[derive(Debug, Clone, Copy)]
pub struct Transition {
    pub prev_state: usize,
    pub input_bit: bool,
    pub output_bits: [bool; 4],
}

pub type StateTransitions = [Transition; 2];

#[derive(Debug, Clone)]
pub struct Trellis {
    pub k: usize,
    pub num_states: usize,
    pub num_outputs: usize,
    pub states: Vec<StateTransitions>
}

pub fn build_trellis() -> Vec<StateTransitions> {

    let mut state = State::new();
    let mut incoming_lists: Vec<Vec<Transition>> = vec![vec![]; 16];

    // Build the states by iterating over the 16 possible "previous" states...
    for prev_state in 0..16 {

        // println!("Processing prev-state {prev_state}");

        // ...and [0, 1] input bits
        for input_bit in [false, true] {

            state.set(prev_state);

            // Compute the output for this state & input bit
            let output_bits = encode_bit(input_bit, &mut state);
            // println!("  Processing input {input_bit} -> output bits {:?} output state {:?}", output_bits, state.val());

            // Add the input & previous state as a possible incoming route to the output state
            incoming_lists[state.val() as usize].push(Transition {
                prev_state: prev_state as usize,
                input_bit,
                output_bits
            })
        }
    }

    // Map those into a list of input pairs, ordered by output state
    incoming_lists
        .into_iter()
        .map(|v| {
            // Should have two transitions
            assert_eq!(v.len(), 2);
            [v[0], v[1]]
        })
        .collect()
}

/// Compute the branch metric for an input codeword and corresponding validity bits
/// For invalid bits, a score of 0 is accumulated.
/// For valid bits, a score of -1 is accumulated for matches or -1 for mismatches.
fn branch_metric_value(input: [bool; 4], validity: [bool; 4], expected: [bool; 4]) -> i32 {
   input
       .iter()
       .zip(validity.iter())
       .zip(expected.iter())
       .map(|((&rx, &valid), &expected)| {

           // If this bit is uncertain (I.e. not valid, the result of depuncturing), return 0
           if !valid {
               return 0;
           }
           
           // Otherwise, return +1 for mismatches or -1 for matches
           return if rx != expected {
               1
           } else {
               -1
           }

       })
       .sum()
}

/// Decode a 1/4-rate convolutionally-coded message
pub fn viterbi_decode(input: Bits, valid_mask: Bits, trellis: &Vec<StateTransitions>) -> Bits {

    // validity mask and input must be the same length
    assert_eq!(input.len(), valid_mask.len());

    // Number of steps must be multiple of 4, since this is as 1/4 rate code
    assert_eq!(input.len() % 4, 0);
    let num_steps = input.len() / 4;

    // Number of states
    let num_states = trellis.len();

    // Track path metrics
    let lowest = i32::MIN / 4;
    let mut prev: [i32; 16] = [lowest; 16];
    let mut current: [i32; 16] = [0; 16];

    // Start in state 0 with cost 0
    prev[0] = 0;

    let mut survivors = vec![[0usize; 16]; num_steps];

    // For each step, which is 4 bits...
    for (step, (input_chunk, valid_chunk)) in
        input.chunks(4).zip(valid_mask.chunks(4)).enumerate() {

        // For each state in the trellis
        for next_state in 0..num_states {

            // There will be two possible incoming routes
            let incoming = &trellis[next_state];
            let mut best_metric = lowest;
            let mut best_prev_state = 0usize;

            // For each of the two transitions
            for trans in incoming {

                // Calculate the branch cost
                let branch_cost = branch_metric_value(
                    [input_chunk[0], input_chunk[1], input_chunk[2], input_chunk[3]],
                    [valid_chunk[0], valid_chunk[1], valid_chunk[2], valid_chunk[3]],
                    trans.output_bits
                );

                // Accumulate the cost
                let total_cost = prev[trans.prev_state] + branch_cost;

                // Improvement?
                if total_cost < best_metric {
                    best_metric = total_cost;
                    best_prev_state = trans.prev_state;
                }
            }

            // Update the current
            current[next_state] = best_metric;

            // Update the survivor
            survivors[step][next_state] = best_prev_state;
        }

        // Swap previous and current
        prev.copy_from_slice(&current);
    }

    // Now we'll backtrace, building the decoded bits
    let mut decoded_bits = Bits::with_capacity(num_steps);
    decoded_bits.resize(num_steps, false); // preallocate space

    // Find the best final state (I.e. the state with the lowest cost in prev)
    let (mut state, _) = prev
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();

    // Work through the steps backwards
    for t in (0..num_steps).rev() {

        // Find the previous state that most likely led to this one
        let prev_state = survivors[t][state];

        // Work out what incoming bit caused this
        let incoming = &trellis[state];
        let trans = incoming
            .iter()
            .find(|transition| transition.prev_state == prev_state)
            .unwrap();

        // Add it to the decoded bits
        decoded_bits.set(t, trans.input_bit);

        // Move on to the next state
        state = prev_state;
    }

    decoded_bits
}

#[cfg(test)]
mod tests {

    use crate::lower::rcpc::coder::{depuncture, rcpc_encode, puncture};
    use crate::lower::rcpc::puncturers::{PredefinedPuncturer, Puncturer};
    use crate::lower::rcpc::viterbi::build_trellis;
    use crate::lower::rcpc::viterbi::viterbi_decode;
    use crate::bits::Bits;
    use crate::bits::from_bitstr;

    #[test]
    fn builds_trellis_correctly() {
        let _trellis = build_trellis();
    }

    #[test]
    fn decodes_simple_correctly() {
        let trellis = build_trellis();
        let example = from_bitstr("00001111101110011110011011100110");
        let valid =  from_bitstr("11111111111111111111111111111111");
        let decoded = viterbi_decode(example, valid, &trellis);
        println!("{}", decoded);
    }

    #[test]
    fn decodes_simple_encoded_example() {

        let trellis = build_trellis();

        // Encode an example
        let example = from_bitstr("11111100");
        let encoded = rcpc_encode(&example, None);
        println!("{}", encoded);

        // Now decode it
        let valid = Bits::repeat(true, encoded.len());
        let decoded = viterbi_decode(encoded, valid, &trellis);
        println!("{}", decoded);

    }

    #[test]
    fn decodes_punctured_encoded_example() {

        let trellis = build_trellis();

        // Encode an example
        let example = from_bitstr("11111100");
        println!("Input:  {} len {}", example, example.len());
        let encoded = rcpc_encode(&example, None);
        println!("Mother: {} len {}", encoded, encoded.len());

        // Puncture the example
        let puncturer = Puncturer::build(&PredefinedPuncturer::Rate1Over3Puncturer);
        let punctured = puncture(&encoded, &puncturer);
        println!("Ra 1/3: {} len {}", punctured, punctured.len());

        // Depuncture the example
        let depunctured = depuncture(&punctured, &puncturer);
        println!("Depunc: {} len {}", depunctured.mother, depunctured.mother.len());
        println!("Valid:  {}", depunctured.valid_mask);

        // Decode
        let decoded = viterbi_decode(depunctured.mother, depunctured.valid_mask, &trellis);
        println!("Decode: {} len {}", decoded, decoded.len());

    }

}