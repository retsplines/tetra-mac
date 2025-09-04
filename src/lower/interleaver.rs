use crate::bits::Bits;

fn interleave_bit(k: usize, a: usize, index: usize) -> usize {
    1 + ((a * index) % k)
}

#[derive(Debug)]
pub enum InterleaverEncodeError {
    InvalidBlockSize { expected: usize, actual: usize },
}

pub fn interleaver_encode(block: &Bits, k: usize, a: usize) -> Result<Bits, InterleaverEncodeError> {

    // If the block isn't exactly k bits in size, error
    if block.len() != k {
        return Err(InterleaverEncodeError::InvalidBlockSize {
            expected: k,
            actual: block.len()
        })
    }

    // For each bit in the block, compute the interleaved index
    let mut interleaved = Bits::repeat(false, k);

    // Iterate over each bit in the block
    for (index, bit) in block.iter().enumerate() {
        let interleaved_index = interleave_bit(k, a, index);
        interleaved.set(interleaved_index - 1, *bit);
    }

    Ok(interleaved)
}

pub fn interleaver_decode(block: &Bits, k: usize, a: usize) -> Bits {

    // For each bit in the block, compute the deinterleaved index
    let mut deinterleaved = Bits::repeat(false, k);

    // Iterate over each bit in the block
    for (index, _bit) in block.iter().enumerate() {
        deinterleaved.set(index, block[interleave_bit(k, a, index) - 1]);
    }

    deinterleaved
}
        
#[cfg(test)]
mod test {

    use bitvec::prelude::*;
    use crate::new_bits;

    #[test]
    fn it_interleaves_bsch_block() {

        // Define a bitvec of 00001111
        let seed = new_bits![0, 0, 0, 0, 1, 1, 1, 1];
        const K: usize = 120;
        const A: usize = 11;
        
        // Multiply up the seed to produce a suitable test block
        let input = seed.repeat(K / seed.len());
        
        // Assert that the block is the correct size
        assert_eq!(input.len(), K);

        // Interleave the block
        let result = super::interleaver_encode(&input, K, A).unwrap();

        // Assert that the length did not change
        assert_eq!(input.len(), result.len());

        // Decode again
        let decoded = super::interleaver_decode(&result, K, A);
        
        // Assert that the decoded block is the same as the input
        assert_eq!(input, decoded);
        
    }
}