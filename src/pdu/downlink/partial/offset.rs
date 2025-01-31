use num_derive::{FromPrimitive, ToPrimitive};
use crate::codec::SizedField;

#[derive(FromPrimitive, ToPrimitive)]
pub enum Offset {
    NoOffset = 0b00,
    Plus6_25kHz = 0b01,
    Minus6_25kHz = 0b10,
    Plus12_5kHz = 0b11
}

impl SizedField for Offset {
    fn size() -> usize {
        2
    }
}
