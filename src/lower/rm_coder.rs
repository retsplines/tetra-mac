use bitvec::prelude::*;
use crate::bits::Bits;
use crate::new_bits;

/// Define the generator matrix rows.
///
/// EN 300 392-2 8.2.3.2
fn generator_matrix_rows() -> Vec<Bits> {
    vec![
        new_bits![1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0],
        new_bits![0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0],
        new_bits![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        new_bits![1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0],
        new_bits![1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0],
        new_bits![0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0],
        new_bits![0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0],
        new_bits![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1],
        new_bits![1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1],
        new_bits![0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1],
        new_bits![0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1],
        new_bits![0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1],
        new_bits![0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1],
        new_bits![0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1]
    ]
}

/// Generate the identity matrix.
/// This basically extends the generator matrix by adding the 14x14 [identity matrix](https://en.wikipedia.org/wiki/Identity_matrix) to the start.
pub fn generate_ident_matrix() -> Vec<Bits> {

    let mut ident_matrix = Vec::new();
    for (index, row) in generator_matrix_rows().iter().enumerate() {

        // Create a row in the identity matrix
        let mut ident_row = new_bits![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        ident_row.shift_right(index);

        // Add the generator matrix row to the identity matrix row
        ident_row.extend_from_bitslice(row);

        // Add the new row to the matrix
        ident_matrix.push(ident_row);
    }

    ident_matrix
}

#[derive(Debug)]
pub enum ReedMullerEncodeError {
    InvalidBlockSize {
        expected: usize,
        actual: usize,
    }
}

/// Encode a 14-bit block using the Reed-Muller coder.
pub fn rm_encode(block: &Bits) -> Result<Bits, ReedMullerEncodeError> {

    // Validate the length of the block
    if block.len() != 14 {
        return Err(ReedMullerEncodeError::InvalidBlockSize {
            expected: 14,
            actual: block.len(),
        })
    }

    // Get the identity matrix
    let ident_matrix = generate_ident_matrix();

    // Encode into 30 bits
    let mut encoded = Bits::with_capacity(30);

    for (index, row) in ident_matrix.iter().enumerate() {
        // If the bit at index is set, xor the row with the corresponding matrix row
        if block[index] {
            encoded ^= row;
        }
    }

    Ok(encoded)
}

#[derive(Debug)]
pub enum ReedMullerDecodeError {
    InvalidBlockSize {
        expected: usize,
        actual: usize,
    },
    MismatchedValue
}

/// Decode a 30-bit block using the Reed-Muller coder.
pub fn rm_decode(block: &Bits) -> Result<Bits, ReedMullerDecodeError> {

    if block.len() != 30 {
        return Err(ReedMullerDecodeError::InvalidBlockSize {
            expected: 30,
            actual: block.len()
        });
    }

    // Extract the payload and code
    let payload = block[0..14].to_bitvec();
    let code = block[14..30].to_bitvec();

    // Validate by re-encoding
    let reencoded = rm_encode(&payload).unwrap();

    // If the re-encoded block does not match the input block, return an error
    if code.eq(&reencoded) {
        return Err(ReedMullerDecodeError::MismatchedValue);
    }

    Ok(payload)
}

#[cfg(test)]
mod test {

    use bitvec::prelude::*;
    use crate::new_bits;

    #[test]
    fn it_generates_ident_matrix() {
        let ident_matrix = super::generate_ident_matrix();
        assert_eq!(ident_matrix.len(), 14);
    }

    #[test]
    fn it_encodes_and_decodes_correctly() {
        let block = new_bits![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
        let encoded = super::rm_encode(&block).unwrap();
        super::rm_decode(&encoded).unwrap();
    }

    #[test]
    fn it_fails_on_incorrect_lengths() {
        let block = new_bits![1, 0, 1, 0];
        assert!(super::rm_encode(&block).is_err());
    }
}
