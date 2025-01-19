use bitvec::prelude::*;
use crate::Bits;

/// Build a PDU sequentially
pub struct Builder {
    data: Bits
}

impl Builder {

    /// Create a new Builder instance
    /// The builder owns the underlying bits during construction
    pub fn new() -> Self {

        Self {
            data: Bits::new()
        }
    }

    /// Write an integer of a defined size to the PDU
    pub fn write_int(&mut self, value: u32, size: usize) {

        // Create a new bit vector sized to fit the new integer
        let mut bv = bitvec![u8, Msb0; 0; size];

        // Fill with all-zero
        bv.fill(false);

        // Store the value
        bv.store_be(value);

        // Append the new data onto the inner slice
        self.data.append(&mut bv);
    }

    /// Write a boolean value to the reader, returning the number of bits written
    pub fn write_bool(&mut self, value: bool) {
        self.data.push(value);
    }

    /// Finish working with the builder, moving out the underlying bits
    pub fn done(self) -> Bits {
        self.data
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_writes_bool_correctly() {

        let mut builder = Builder::new();

        // True, then false, then true
        builder.write_bool(true);
        builder.write_bool(false);
        builder.write_bool(true);

        // Validate the bits are correct
        assert_eq!(builder.data.as_bitslice(), bitvec![1, 0, 1]);

        // Length should be exact
        assert_eq!(builder.data.len(), 3);
    }

    #[test]
    fn it_writes_int_correctly() {

        let mut builder = Builder::new();

        // Write integers
        builder.write_int(16, 6);
        builder.write_int(1023, 10);

        // Length?
        assert_eq!(builder.data.len(), 16);
        assert_eq!(builder.data.as_raw_slice(), [0x43, 0xFF]);

    }

}