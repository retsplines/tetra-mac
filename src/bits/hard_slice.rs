use std::fmt::Formatter;
use ref_cast::RefCast;

/// Slice into "hard" bits (1 or 0)
#[derive(Debug, RefCast)]
#[repr(transparent)]
pub struct HardBitSlice(pub [bool]);

impl std::fmt::Display for HardBitSlice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in self.0.iter() {
            write!(f, "{}", if *i { "1" } else { "0" })?;
        }
        write!(f, " ({})", self.0.len())?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum StoreError {
    Overflowed
}

impl HardBitSlice {

    /// Given a value, attempt to store it in the mutable slice.
    /// If the value doesn't fit, Overflowed will be returned.
    pub fn store(&mut self, value: u32) -> Result<(), StoreError> {

        let mut remainder = value;

        for bit in self.0.iter_mut() {
            *bit = (remainder & 1) != 0;
            remainder >>= 1;
        }

        // If we're at the last bit of the slice but have more value to store, we overflowed
        if remainder != 0 {
            return Err(StoreError::Overflowed);
        }

        Ok(())
    }

    /// Check that all bits are set
    pub fn all_set(&self) -> bool {
        self.0.iter().all(|&b| b)
    }

    /// Check that all bits are clear
    pub fn all_clear(&self) -> bool {
        self.0.iter().all(|&b| !b)
    }
}

#[cfg(test)]
mod tests {
    use crate::bits::hard_slice::StoreError::Overflowed;
    use crate::bits::hard_vec::HardBitVec;

    #[test]
    fn store_in_slice_works() {
        let mut hb1: HardBitVec = "0000 0000 0000".parse().unwrap();
        hb1[4..8].store(0b1111).unwrap();
        assert!(hb1[0..4].all_clear());
        assert!(hb1[4..8].all_set());
        assert!(hb1[8..12].all_clear());
    }

    #[test]
    fn store_in_slice_overflows() {
        let mut hb1: HardBitVec = "0000".parse().unwrap();
        assert_eq!(hb1[0..4].store(0b10000), Err(Overflowed));
    }
}