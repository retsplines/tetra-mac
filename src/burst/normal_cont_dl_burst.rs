use phase_adjustment::phase_adjustment_bits;
use crate::bits::Bits;
use crate::burst::partial::training_sequence::{
    training_sequence_normal_1_bits, 
    training_sequence_normal_2_bits, 
    training_sequence_normal_3_bits
};
use crate::burst::partial::phase_adjustment;
use crate::burst::partial::phase_adjustment::{extract_sn_range_bits, PHASE_ADJUSTMENT_SYMBOL_RANGE_HA, PHASE_ADJUSTMENT_SYMBOL_RANGE_HB};

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
    // TF1's PA bits are defined by "HA"
    let tf1_pa_bits = phase_adjustment_bits(
        &extract_sn_range_bits(&burst, PHASE_ADJUSTMENT_SYMBOL_RANGE_HA.0, PHASE_ADJUSTMENT_SYMBOL_RANGE_HA.1)
    );

    // TF2's PA bits are defined by "HB"
    let tf2_pa_bits = phase_adjustment_bits(
        &extract_sn_range_bits(&burst, PHASE_ADJUSTMENT_SYMBOL_RANGE_HB.0, PHASE_ADJUSTMENT_SYMBOL_RANGE_HB.1)
    );
    
    log::info!("computed TF1 PA = {}{}", tf2_pa_bits[0], tf2_pa_bits[1]);

    // Insert the TF PA bits into the structure
    burst.splice(tf1_pa_ref ..tf1_pa_ref + 2, tf1_pa_bits);
    burst.splice(tf2_pa_ref  .. tf2_pa_ref + 2, tf2_pa_bits);

    burst
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::bits::Bits;

    #[test]
    fn empty_burst_is_correct() {

        let burst = build_burst_normal_cont_dl(
            Bits::repeat(false, 216),
            Bits::repeat(false, 216),
            Bits::repeat(false, 30),
            false
        );

        print!("burst {burst}");

    }



}