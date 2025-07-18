use bitvec::prelude::*;

fn interleave_bit(k: u32, a: u32, index: u32) -> u32 {
    1 + ((a * index) % k)
}

pub fn encode(block: &BitVec, k: u32, a: u32) -> Result<BitVec, &'static str> {

    // If the block isn't exactly k bits in size, error
    if block.len() != k as usize {
        return Err("Block size must be exactly k bits");
    }

    // For each bit in the block, compute the interleaved index
    let mut interleaved = bitvec![0; k as usize];

    // Iterate over each bit in the block
    for (index, bit) in block.iter().enumerate() {
        let interleaved_index = interleave_bit(k, a, index as u32);
        interleaved.set((interleaved_index - 1) as usize, *bit);
    }

    Ok(interleaved)
}

pub fn decode(block: &BitVec, k: u32, a: u32) -> Result<BitVec, &'static str> {

    // For each bit in the block, compute the deinterleaved index
    let mut deinterleaved = bitvec![0; k as usize];

    // Iterate over each bit in the block
    for (index, _bit) in block.iter().enumerate() {
        deinterleaved.set(index, block[interleave_bit(k, a, index as u32) as usize - 1]);
    }

    Ok(deinterleaved)
}
        
#[cfg(test)]
mod test {

    use bitvec::bitvec;
    use bitvec::prelude::*;
    
    #[test]
    fn it_interleaves_bsch_block() {

        // Define a bitvec of 00001111
        let seed = bitvec![0, 0, 0, 0, 1, 1, 1, 1];
        const K: u32 = 120;
        const A: u32 = 11;
        
        // Multiply up the seed to produce a suitable test block
        let input = seed.repeat(K as usize / seed.len());
        
        // Assert that the block is the correct size
        assert_eq!(input.len(), K as usize);

        // Interleave the block
        let result = super::encode(&input, K, A).unwrap();

        // Assert that the length did not change
        assert_eq!(input.len(), result.len());

        // Decode again
        let decoded = super::decode(&result, K, A).unwrap();
        
        // Assert that the decoded block is the same as the input
        assert_eq!(input, decoded);
        
    }
}