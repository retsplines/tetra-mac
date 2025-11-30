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

impl SyncContDownlinkBurst {

    /// Builds the synchronisation continuous downlink burst
    pub fn build(&self) -> Bits {

        // Validate the lengths of the blocks
        if self.sb1_bits.len() != 120 {
            panic!("sb1 block length for normal cont downlink burst must be 120 bits got {}", self.sb1_bits.len())
        }

        if self.sb2_bits.len() != 216 {
            panic!("sb2 block length for normal cont downlink burst must be 216 bits got {}", self.sb2_bits.len())
        }

        if self.bb_bits.len() != 30 {
            panic!("broadcast block length for normal cont downlink burst must be 30 bits got {}", self.bb_bits.len())
        }

        // Build the burst
        let mut burst_bits = Bits::new();

        // Training sequence 3 (q11-q22)
        burst_bits.extend(&training_sequence_normal_3_bits()[(11 - 1)..22]);

        // Placeholder bits phase adjustment A
        let pa_a_ref = burst_bits.len();
        burst_bits.extend([false; 2]);

        // Frequency correction bits (f1-f80)
        burst_bits.extend(&frequency_correction_bits()[0..80]);

        // SB1
        burst_bits.extend(&self.sb1_bits);

        // Synchronisation training sequence (y1-38)
        burst_bits.extend(&training_sequence_sync_bits()[0..38]);

        // BB (30 bits)
        burst_bits.extend(&self.bb_bits[0..30]);

        // SB2
        burst_bits.extend(&self.sb2_bits);

        // Placeholder bits phase adjustment B
        let pa_b_ref = burst_bits.len();
        burst_bits.extend([false; 2]);

        // Training sequence 3 (q1-q10)
        burst_bits.extend(&training_sequence_normal_3_bits()[0..10]);

        // Calculate the phase adjustment fields
        // Phase Adjustment A bits are defined by "HC"
        let pa_a_bits = phase_adjustment_bits(
            &extract_sn_range_bits(&burst_bits, PHASE_ADJUSTMENT_SYMBOL_RANGE_HC.0, PHASE_ADJUSTMENT_SYMBOL_RANGE_HC.1)
        );

        // Phase Adjustment B bits are defined by "HD"
        let pa_b_bits = phase_adjustment_bits(
            &extract_sn_range_bits(&burst_bits, PHASE_ADJUSTMENT_SYMBOL_RANGE_HD.0, PHASE_ADJUSTMENT_SYMBOL_RANGE_HD.1)
        );

        // Insert the A and B phase adjustment bits into the structure
        burst_bits.splice(pa_a_ref .. pa_a_ref + 2, pa_a_bits);
        burst_bits.splice(pa_b_ref .. pa_b_ref + 2, pa_b_bits);

        burst_bits
    }

    /// Validate and extract the synchronisation continuous downlink burst
    pub fn extract(burst: Bits) -> Result<SyncContDownlinkBurst, BurstExtractionError> {

        if burst.len() != 510 {
            return Err(BurstExtractionError::IncorrectLength {
                expected: 510,
                provided: burst.len()
            })
        }

        Ok(SyncContDownlinkBurst {
            sb1_bits: Bits::from_bitslice(&burst[94..214]),
            bb_bits: Bits::from_bitslice(&burst[252..282]),
            sb2_bits: Bits::from_bitslice(&burst[282..498])
        })
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::bits::Bits;
    use crate::bits::from_bitstr;

    #[test]
    fn empty_burst_is_correct() {

        let burst = SyncContDownlinkBurst {
            sb1_bits: Bits::repeat(false, 120),
            sb2_bits: Bits::repeat(false, 216),
            bb_bits: Bits::repeat(false, 30)
        };

        let burst_bits = burst.build();

        assert_eq!(burst_bits.len(), 510);

    }

    #[test]
    fn extracts_burst_correctly() {

        let burst_bits = from_bitstr("
           000110101101  // sync
           10  // pa
           11111111000000000000000000000000000000000000000000000000000000000000000011111111  // frequency // correct.
           000110010011001011111001100110000100100000110101101010000110110100110110110100100001001000111010010111100110101110101011  // sb
           11000001100111001110100111000001100111  // sync / training
           101111101111011100001110111001  // bb
           101111111110010011110001100110101110000101100111001001100010111010100001101000101111100000001111101111110101101000111001010011011101010010010100011000010010100100111001101000100101011011010101101011101010011001011110  // sb
           01  // pa
           1011011100  // sync
        ");

        // Extract the burst
        let extracted_burst = SyncContDownlinkBurst::extract(burst_bits).unwrap();

        println!("{:?}", extracted_burst.sb1_bits.len());
        println!("{:?}", extracted_burst.bb_bits.len());
        println!("{:?}", extracted_burst.sb2_bits.len());
    }

}