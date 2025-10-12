use std::fmt::{Debug, Formatter};
use std::ops::{Index, Range, RangeFull};
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
}

impl std::ops::Index<Range<usize>> for HardBitVec {
    type Output = HardBitSlice;
    fn index(&self, range: Range<usize>) -> &Self::Output {
        HardBitSlice::ref_cast(&self.0[range])
    }
}

impl Index<RangeFull> for HardBitVec {
    type Output = HardBitSlice;

    fn index(&self, range: RangeFull) -> &Self::Output {
        HardBitSlice::ref_cast(&self.0[range])
    }
}

mod tests {

    use std::str::FromStr;
    use crate::bits::hard_vec::HardBitVec;

    #[test]
    fn from_str_works() {

        let mut hb1 = HardBitVec::from_str("010010100101").unwrap();
        println!("{}", hb1);

        println!("{}", &hb1[..]);


    }

}