use bitvec::prelude::*;

pub struct Cursor<'a> {
    slice: &'a BitSlice<u8>,
    position: usize
}

impl<'a> Cursor<'a> {

    /// Create a new Cursor instance
    pub fn new(source: &'a BitSlice<u8>) -> Self {
        Self {
            slice: source,
            position: 0
        }
    }

    /// Get the number of remaining bits
    fn remaining(&self) -> usize {
        self.slice.len() - self.position
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
        let val = self.slice[self.position .. (self.position + size)].load::<u32>();

        // Advance the cursor
        self.position += size;

        val
    }

    /// Read an optional integer, which may be up to 32 bits
    /// The presence of the field is indicated by a preceeding O-bit
    pub fn read_int_optional(&mut self, size: usize) -> Option<u32> {

        // Read the O-bit
        if self.remaining() < 1 {
            panic!("attempting to read O-bit while at end of buffer")
        }

        let o_bit = self.slice[self.position];
        self.position += 1;

        if !o_bit {
            return None
        }

        // We're reading the field
        Some(self.read_int(size))
    }

    /// Read the next bit as a boolean
    pub fn read_bool(&mut self) -> bool {

        if self.remaining() < 1 {
            panic!("attempting to read a boolean while at end of buffer")
        }

        // Return the bit without advancing
        let result = self.slice[self.position];
        self.position += 1;

        result
    }

}