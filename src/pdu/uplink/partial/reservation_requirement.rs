#[derive(Debug)]
pub enum ReservationRequirement {
    Subslot = 0b000,
    Slot = 0b0001,
    Slots2 = 0b0010,
    Slots3 = 0b0011,
    Slots4 = 0b0100,
    Slots5 = 0b0101,
    Slots6 = 0b0110,
    Slots8 = 0b0111,
    Slots10 = 0b1000,
    Slots13 = 0b1001,
    Slots17 = 0b1010,
    Slots24 = 0b1011,
    Slots34 = 0b1100,
    Slots51 = 0b1101,
    Slots68 = 0b1110,
    MoreThan68Slots = 0b1111
}
