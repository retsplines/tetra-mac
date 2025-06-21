use phase_adjustment::phase_adjustment_bits;
use crate::Bits;
use crate::burst::partial::training_sequence::{
    training_sequence_normal_1_bits, 
    training_sequence_normal_2_bits, 
    training_sequence_normal_3_bits
};
use crate::burst::partial::phase_adjustment;

/// Builds the normal continuous downlink burst
fn build_burst_normal_cont_dl(
    sb1_bits: Bits,
    sb2_bits: Bits,
    bb_bits: Bits,
    slot_flag: bool
) -> Bits {

    // Validate the lengths of the blocks
    if sb1_bits.len() != 216 {
        panic!("sb1 block length for normal cont downlink burst must be 216 bits got {}", sb1_bits.len())
    }

    if sb2_bits.len() != 216 {
        panic!("sb2 block length for normal cont downlink burst must be 216 bits got {}", sb2_bits.len())
    }

    if bb_bits.len() != 30 {
        panic!("broadcast block length for normal cont downlink burst must be 30 bits got {}", bb_bits.len())
    }

    // Build the burst
    let mut burst = Bits::new();

    // Training sequence 3 (q11-q22)
    burst.extend(&training_sequence_normal_3_bits()[(11 - 1)..22]);

    // Placeholder bits for training field 1's phase adjustment bits
    let tf1_pa_ref = burst.len();
    burst.extend([false; 2]);

    // SB1
    burst.extend(sb1_bits);

    // BB (14 bits)
    burst.extend(&bb_bits[0..14]);

    // Training sequence 2 if the slot flag is present, otherwise training sequence 1
    if slot_flag {
        // p1-p22
        burst.extend(&training_sequence_normal_2_bits()[0..22]);
    } else {
        // n1-n22
        burst.extend(&training_sequence_normal_1_bits()[0..22]);
    }

    // BB (16 bits)
    burst.extend(&bb_bits[14..30]);

    // SB2
    burst.extend(sb2_bits);

    // Placeholder bits for training field 2's phase adjustment bits
    let tf2_pa_ref = burst.len();
    burst.extend([false; 2]);

    // Training sequence 3 (q1-q10)
    burst.extend(&training_sequence_normal_3_bits()[0..10]);

    // Calculate the phase adjustment fields
    let tf1_pa_bits = phase_adjustment_bits(
        
    )


    burst
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::Bits;
    use bitvec::prelude::*;

    #[test]
    fn empty_burst_is_correct() {

        let burst = build_burst_normal_cont_dl(
            Bits::repeat(false, 216),
            Bits::repeat(false, 216),
            Bits::repeat(false, 30),
            false
        );

        print!("burst {}", burst);

    }



}