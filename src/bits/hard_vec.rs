use std::fmt::{Debug, Formatter};
use std::ops::Index;
use std::ops::Range;
use std::ops::RangeFull;
use crate::bits::hard_slice::HardBitSlice;
use ref_cast::RefCast;

/// Storage of "hard" bits (1 or 0) as a Vec of booleans
#[derive(Debug)]
#[repr(transparent)]
pub struct HardBitVec(Vec<bool>);

impl std::str::FromStr for HardBitVec {
    type Err = &'static str;

    /// Quick conversion of strings of 0101 to HardBits
    /// Characters other than 0 or 1 will be ignored, so can be used for formatting
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits: Result<Vec<bool>, _> = s.chars()
            .filter(|c| *c == '0' || *c == '1')
            .map(|c| {
                match c {
                    '0' => Ok(false),
                    '1' => Ok(true),
                    _ => Err("Invalid character"),
                }
            })
            .collect();
        bits.map(HardBitVec)
    }
}

impl std::ops::Deref for HardBitVec {
    type Target = Vec<bool>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for HardBitVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Delegate to the slice implementation
        std::fmt::Display::fmt(&HardBitSlice::ref_cast(&self.0[..]), f)
    }
}

impl HardBitVec {

    /// Push an integer of the specified size into the vector
    pub fn push_int(&mut self, value: u64, bits: usize) {
        for i in (0..bits).rev() {
            let bit = (value >> i) & 1 != 0;
            self.0.push(bit);
        }
    }

    pub fn new(size: usize) -> Self {
        HardBitVec(vec![false; size])
    }
}

impl std::ops::Index<Range<usize>> for HardBitVec {
    type Output = HardBitSlice;
    fn index(&self, range: Range<usize>) -> &Self::Output {
        HardBitSlice::ref_cast(&self.0[range])
    }
}

impl std::ops::IndexMut<Range<usize>> for HardBitVec {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        HardBitSlice::ref_cast_mut(&mut self.0[range])
    }
}

impl Index<RangeFull> for HardBitVec {
    type Output = HardBitSlice;
    fn index(&self, range: RangeFull) -> &Self::Output {
        HardBitSlice::ref_cast(&self.0[range])
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn from_str_works() {
        let hb1: HardBitVec = "010010100101".parse().unwrap();
        assert_eq!(hb1.len(), 12);
        assert_eq!(hb1.0[0], false);
        assert_eq!(hb1.0[11], true);
    }

    #[test]
    fn push_int_works() {
        let mut hb1: HardBitVec = HardBitVec::new(0);
        hb1.push_int(1, 4);
        println!("{}", hb1);
    }

}