use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, PartialEq, FromPrimitive, ToPrimitive)]
pub enum TSReservedFrames {
    Reserve1 = 0b000,
    Reserve2 = 0b001,
    Reserve3 = 0b010,
    Reserve4 = 0b011,
    Reserve6 = 0b100,
    Reserve9 = 0b101,
    Reserve12 = 0b110,
    Reserve18 = 0b111
}
