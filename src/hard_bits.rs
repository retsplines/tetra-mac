#[derive(Debug)]
pub struct HardBits(Vec<bool>);

impl std::str::FromStr for HardBits {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits: Result<Vec<bool>, _> = s.chars().map(|c| {
            match c {
                '0' => Ok(false),
                '1' => Ok(true),
                _ => Err("Invalid character"),
            }
        }).collect();
        bits.map(HardBits)
    }
}

impl std::ops::Index<usize> for HardBits {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::Index<std::ops::Range<usize>> for HardBits {
    type Output = [bool];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::Index<std::ops::RangeTo<usize>> for HardBits {
    type Output = [bool];

    fn index(&self, index: std::ops::RangeTo<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::Index<std::ops::RangeFrom<usize>> for HardBits {
    type Output = [bool];

    fn index(&self, index: std::ops::RangeFrom<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::Index<std::ops::RangeFull> for HardBits {
    type Output = [bool];

    fn index(&self, index: std::ops::RangeFull) -> &Self::Output {
        &self.0[index]
    }
}

impl HardBits {
    pub fn push_int(&mut self, value: u64, bits: usize) {
        for i in (0..bits).rev() {
            let bit = (value >> i) & 1 != 0;
            self.0.push(bit);
        }
    }
}


mod tests {
    use std::str::FromStr;
    use crate::hard_bits::HardBits;

    #[test]
    fn works() {

        let hb1 = HardBits::from_str("010010100101").unwrap();
        println!("{:?}", hb1);

        let bit1 = hb1[0];

        let range1 = &hb1[0..3];
        println!("{:?}", range1);

    }

}