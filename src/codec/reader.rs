use bitvec::prelude::*;
use crate::bits::Bits;

pub struct Reader<'a> {
    data: &'a Bits,
    position: usize
}

impl<'a> Reader<'a> {

    /// Create a new reader instance
    pub fn new(source: &'a Bits) -> Self {

        Self {
            data: source,
            position: 0
        }
    }

    /// Panic due to a bounds check failure
    fn bounds_check_fail(&self, callee: &str, size: usize) -> ! {
        panic!(
            "{}: Attempting to read {size} bits at position {}, but only {} bits remaining, total size is {}",
            callee, self.position, self.count_remaining(), self.data.len()
        )
    }

    /// Get the number of remaining bits
    pub fn count_remaining(&self) -> usize {
        self.data.len() - self.position
    }
    
    /// Skip a number of bits
    pub fn skip(&mut self, size: usize) {

        if size > self.count_remaining() {
            self.bounds_check_fail("skip", size);
        }

        // Advance the reader
        self.position += size;
    }

    /// Read an integer, which may be up to 32 bits
    pub fn read_int(&mut self, size: usize) -> u32 {

        if size > 32 {
            panic!("can't read more than 32 bits, attempting {size}")
        }

        let remaining= self.count_remaining();
        if size > remaining {
            self.bounds_check_fail("read_int", size);
        }

        // Read the bits & load into a u32
        let val = self.data[self.position .. (self.position + size)].load_be::<u32>();

        // Advance the reader
        self.position += size;

        val
    }

    /// Read the next bit as a boolean
    pub fn read_bool(&mut self) -> bool {

        if self.count_remaining() < 1 {
            self.bounds_check_fail("read_bool", 1);
        }

        // Return the bit without advancing
        let result = self.data[self.position];
        self.position += 1;

        result
    }

    /// Read the rest of the bits that are available and return them inside a BitVec
    pub fn read_rest(&mut self) -> Bits {

        // Count the remaining bits
        let remaining = self.count_remaining();

        // Create a new BitVec with the remaining bits
        let mut result = Bits::with_capacity(remaining);

        // Copy the remaining bits into the new BitVec
        result.extend_from_bitslice(&self.data[self.position .. self.position + remaining]);

        // Advance the reader
        self.position += remaining;

        result
    }

    pub fn read(&mut self, size: usize) -> Bits {

        if size > self.count_remaining() {
            self.bounds_check_fail("read", size);
        }

        // Read the bits and return them
        let result = Bits::from_bitslice(&self.data[self.position .. self.position + size]);

        // Advance the reader
        self.position += size;

        result
    }
}


#[cfg(test)]
mod tests {
    use crate::new_bits;
    use bitvec::prelude::*;
    use super::*;

    #[test]
    fn reads_6_bit_int() {

        let data = new_bits![
            1, 1, 0, 0, 0, 0, // 48
            1, 1, 0, 0, 0, 0, // 48
            0, 0, 0, 0, 0, 0, // Padding bits
        ];


        // Create a reader over the data
        let mut cur = Reader::new(&data);

        assert_eq!(cur.read_int(6), 48);
        assert_eq!(cur.read_int(6), 48);
    }

    #[test]
    fn reads_bool() {

        let data = new_bits![
            0, 1, 0, 1
        ];

        // Create a reader over the data
        let mut cur = Reader::new(&data);

        // Read the first bit
        assert!(!cur.read_bool());
        assert!(cur.read_bool());
        assert!(!cur.read_bool());
        assert!(cur.read_bool());
    }
}