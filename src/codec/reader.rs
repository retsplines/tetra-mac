use bitvec::prelude::*;
use crate::Bits;

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

    /// Get the number of remaining bits
    fn remaining(&self) -> usize {
        self.data.len() - self.position
    }

    /// Read an integer, which may be up to 32 bits
    pub fn read_int(&mut self, size: usize) -> u32 {

        if size > 32 {
            panic!("can't read more than 32 bits, attempting {size}")
        }

        let remaining= self.remaining();
        if size > remaining {
            panic!("not enough bits left, need {size} got {remaining}")
        }

        // Read the bits & load into a u32
        let val = self.data[self.position .. (self.position + size)].load_be::<u32>();

        // Advance the reader
        self.position += size;

        val
    }

    /// Read the next bit as a boolean
    pub fn read_bool(&mut self) -> bool {

        if self.remaining() < 1 {
            panic!("attempting to read a boolean while at end of buffer")
        }

        // Return the bit without advancing
        let result = self.data[self.position];
        self.position += 1;

        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_6_bit_int() {

        let data: Bits = Bits::from_vec(vec![
            0b110000_11, 0b0000_0000
        ]);

        // Create a reader over the data
        let mut cur = Reader::new(&data);

        let first_int = cur.read_int(6);
        assert_eq!(first_int, 48);

        // Read another
        let second_int = cur.read_int(6);
        assert_eq!(second_int, 48);

    }
}