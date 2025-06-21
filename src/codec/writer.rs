use bitvec::prelude::*;
use crate::Bits;

/// Writes a PDU sequentially
pub struct Writer {
    data: Bits
}

impl Writer {

    /// Create a new Writer instance
    /// The writer owns the underlying bits during construction
    pub fn new() -> Self {

        Self {
            data: Bits::new()
        }
    }

    /// Write an integer of a defined size
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

    /// Write a boolean value, returning the number of bits written
    pub fn write_bool(&mut self, value: bool) {
        self.data.push(value);
    }

    /// Finish working with the writer, moving out the underlying bits
    pub fn done(self) -> Bits {
        self.data
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_bool() {

        let mut writer = Writer::new();

        // True, then false, then true
        writer.write_bool(true);
        writer.write_bool(false);
        writer.write_bool(true);

        // Validate the bits are correct
        assert_eq!(writer.data.as_bitslice(), bitvec![1, 0, 1]);

        // Length should be exact
        assert_eq!(writer.data.len(), 3);
    }

    #[test]
    fn writes_int() {

        let mut writer = Writer::new();

        // Write integers
        writer.write_int(16, 6);
        writer.write_int(1023, 10);

        // Length?
        assert_eq!(writer.data.len(), 16);
        assert_eq!(writer.data.as_raw_slice(), [0x43, 0xFF]);

    }

}