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

pub enum StoreError {
    Overflowed
}

impl HardBitSlice {

    pub fn store(&mut self, value: u32) -> Result<(), StoreError> {

        let mut remainder = value;

        

        Ok(())
    }

}
