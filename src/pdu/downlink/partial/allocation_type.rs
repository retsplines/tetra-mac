use num_derive::{FromPrimitive, ToPrimitive};
use crate::codec::SizedField;

#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum AllocationType {
    Replacement = 0b00,
    Addition = 0b01,
    QuitAndGoTo = 0b10,
    ReplacePlus = 0b11
}

impl SizedField for AllocationType {
    fn size() -> usize {
        2
    }
}
