use phase_adjustment::phase_adjustment_bits;
use crate::bits::Bits;
use crate::burst::partial::training_sequence::{
    training_sequence_normal_3_bits,
    training_sequence_sync_bits
};
use crate::burst::partial::frequency_correction::{
    frequency_correction_bits
};
use crate::burst::partial::phase_adjustment;
use crate::burst::partial::phase_adjustment::{
    extract_sn_range_bits,
    PHASE_ADJUSTMENT_SYMBOL_RANGE_HC,
    PHASE_ADJUSTMENT_SYMBOL_RANGE_HD
};

/// Builds the synchronisation continuous downlink burst
fn build_burst_sync_cont_dl(
    sb1_bits: Bits,
    sb2_bits: Bits,
    bb_bits: Bits
) -> Bits {

    // Validate the lengths of the blocks
    if sb1_bits.len() != 120 {
        panic!("sb1 block length for normal cont downlink burst must be 120 bits got {}", sb1_bits.len())
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

    // Placeholder bits phase adjustment A
    let pa_a_ref = burst.len();
    burst.extend([false; 2]);

    // Frequency correction bits (f1-f80)
    burst.extend(&frequency_correction_bits()[0..80]);

    // SB1
    burst.extend(sb1_bits);

    // Synchronisation training sequence (y1-38)
    burst.extend(&training_sequence_sync_bits()[0..38]);

    // BB (30 bits)
    burst.extend(&bb_bits[0..30]);

    // SB2
    burst.extend(sb2_bits);

    // Placeholder bits phase adjustment B
    let pa_b_ref = burst.len();
    burst.extend([false; 2]);

    // Training sequence 3 (q1-q10)
    burst.extend(&training_sequence_normal_3_bits()[0..10]);

    // Calculate the phase adjustment fields
    // Phase Adjustment A bits are defined by "HC"
    let pa_a_bits = phase_adjustment_bits(
        &extract_sn_range_bits(&burst, PHASE_ADJUSTMENT_SYMBOL_RANGE_HC.0, PHASE_ADJUSTMENT_SYMBOL_RANGE_HC.1)
    );

    // Phase Adjustment B bits are defined by "HD"
    let pa_b_bits = phase_adjustment_bits(
        &extract_sn_range_bits(&burst, PHASE_ADJUSTMENT_SYMBOL_RANGE_HD.0, PHASE_ADJUSTMENT_SYMBOL_RANGE_HD.1)
    );

    log::info!("computed PA A = {}{}", pa_a_bits[0], pa_a_bits[1]);
    log::info!("computed PA B = {}{}", pa_b_bits[0], pa_b_bits[1]);

    // Insert the A and B phase adjustment bits into the structure
    burst.splice(pa_a_ref .. pa_a_ref + 2, pa_a_bits);
    burst.splice(pa_b_ref  .. pa_b_ref + 2, pa_b_bits);

    burst
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::bits::Bits;

    #[test]
    fn empty_burst_is_correct() {

        let _ = env_logger::builder().is_test(true).try_init();

        let burst = build_burst_sync_cont_dl(
            Bits::repeat(false, 120),
            Bits::repeat(false, 216),
            Bits::repeat(false, 30),
        );

        print!("burst {burst}");

    }



}