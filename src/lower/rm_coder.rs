use bitvec::prelude::*;

/// Define the generator matrix rows.
///
/// EN 300 392-2 8.2.3.2
fn generator_matrix_rows() -> Vec<BitVec> {
    vec![
        bitvec![1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0],
        bitvec![0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0],
        bitvec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        bitvec![1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
        bitvec![1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0],
        bitvec![0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0],
        bitvec![0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0],
        bitvec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1],
        bitvec![1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1],
        bitvec![0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1],
        bitvec![0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1],
        bitvec![0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1],
        bitvec![0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1],
        bitvec![0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1]
    ]
}

/// Generate the identity matrix.
/// This basically extends the generator matrix by adding the 14x14 [identity matrix](https://en.wikipedia.org/wiki/Identity_matrix) to the start.
pub fn generate_ident_matrix() -> Vec<BitVec> {

    let mut ident_matrix = Vec::new();
    for (index, row) in generator_matrix_rows().iter().enumerate() {

        // Create a row in the identity matrix
        let mut ident_row = bitvec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        ident_row.shift_right(index);

        // Add the generator matrix row to the identity matrix row
        ident_row.extend_from_bitslice(row);

        // Add the new row to the matrix
        ident_matrix.push(ident_row);
    }

    ident_matrix
}


/// Encode a 14-bit block using the Reed-Muller coder.
pub fn encode(block: &BitVec) -> Result<BitVec, &'static str> {

    // Validate the length of the block
    if block.len() != 14 {
        return Err("30,14 RM coder only operates on 14-bit blocks");
    }

    // Get the identity matrix
    let ident_matrix = generate_ident_matrix();

    // Encode into 30 bits
    let mut encoded = bitvec![0; 30];

    for (index, row) in ident_matrix.iter().enumerate() {
        // If the bit at index is set, xor the row with the corresponding matrix row
        if block[index] {
            encoded ^= row;
        }
    }

    Ok(encoded)
}

/// Decode a 30-bit block using the Reed-Muller coder.
pub fn decode(block: &BitVec) -> Result<BitVec, &'static str> {

    if block.len() != 30 {
        return Err("30,14 RM decoder only operates on 30-bit blocks");
    }

    // Extract the payload and code
    let payload = block[0..14].to_bitvec();
    let code = block[14..30].to_bitvec();

    // Validate by re-encoding
    let reencoded = encode(&payload).unwrap();

    // If the re-encoded block does not match the input block, return an error
    if code.eq(&reencoded) {
        return Err("Reed-Muller block does not match input block");
    }

    Ok(payload)
}

#[cfg(test)]
mod test {

    use bitvec::prelude::*;

    #[test]
    fn it_generates_ident_matrix() {
        let ident_matrix = super::generate_ident_matrix();
        assert_eq!(ident_matrix.len(), 14);
    }

    #[test]
    fn it_encodes_and_decodes_correctly() {

        let block = bitvec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        let encoded = super::encode(&block).unwrap();
        super::decode(&encoded).unwrap();

    }

    #[test]
    fn it_fails_on_incorrect_lengths() {
        let block = bitvec![1, 0, 1, 0];
        assert!(super::encode(&block).is_err());
    }

}