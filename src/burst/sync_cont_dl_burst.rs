use phase_adjustment::phase_adjustment_bits;
use crate::bits::Bits;
use crate::burst::BurstExtractionError;
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

#[derive(Debug)]
pub struct SyncContDownlinkBurst {
    sb1_bits: Bits,
    sb2_bits: Bits,
    bb_bits: Bits
}

/// Builds the synchronisation continuous downlink burst
pub fn build_burst_sync_cont_dl(
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
    burst.splice(pa_b_ref .. pa_b_ref + 2, pa_b_bits);

    burst
}

/// Validate and extract the synchronisation continuous downlink burst
pub fn extract_burst_sync_cont_dl(burst: Bits) -> Result<SyncContDownlinkBurst, BurstExtractionError> {

    if burst.len() != 510 {
        return Err(BurstExtractionError::IncorrectLength {
            expected: 510,
            provided: burst.len()
        })
    }

    Ok(SyncContDownlinkBurst {
        sb1_bits: Bits::from_bitslice(&burst[94..214]),
        sb2_bits: Bits::from_bitslice(&burst[252..282]),
        bb_bits: Bits::from_bitslice(&burst[282..498])
    })
}

#[cfg(test)]
mod tests {
    use bitvec::order::Msb0;
    use bitvec::slice::BitSlice;
    use bitvec::view::BitView;
    use super::*;
    use crate::bits::Bits;

    #[test]
    fn empty_burst_is_correct() {

        let burst = build_burst_sync_cont_dl(
            Bits::repeat(false, 120),
            Bits::repeat(false, 216),
            Bits::repeat(false, 30),
        );

        assert_eq!(burst.len(), 510);

    }

    #[test]
    #[ignore]
    fn extracts_burst_correctly() {

        // TODO: Too long - find out what excess bits we have here...
        let burst_bitslice: &BitSlice<u8, Msb0> = [
            0x1a, 0xdb, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xfc, 0x64, 0xcb, 0xe6, 0x61,
            0x20, 0xd6, 0xa1, 0xb4, 0xdb, 0x48, 0x48, 0xe9, 0x79, 0xae, 0xaf, 0x06, 0x73, 0xa7, 0x06, 0x7b,
            0xef, 0x70, 0xee, 0x6f, 0xf9, 0x3c, 0x66, 0xb8, 0x59, 0xc9, 0x8b, 0xa8, 0x68, 0xbe, 0x03, 0xef,
            0xd6, 0x8e, 0x53, 0x75, 0x25, 0x18, 0x4a, 0x4e, 0x68, 0x95, 0xb5, 0x6b, 0xa9, 0x97, 0x9b, 0x70,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0xaa, 0xaa, 0x00, 0x04, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d,
            0x00, 0x33, 0x00, 0x00, 0x00, 0x00, 0xab, 0x00, 0x27, 0x59
        ].view_bits::<Msb0>();


        let burst_bits = Bits::from_bitslice(burst_bitslice);

        // Extract the burst
        let extracted_burst = extract_burst_sync_cont_dl(burst_bits);

        println!("{:?}", extracted_burst.unwrap().bb_bits.len());

    }





}