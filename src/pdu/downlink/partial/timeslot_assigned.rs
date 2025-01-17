use crate::codec::{Cursor, Decodable};

#[derive(Debug)]
pub enum TimeslotAssigned {
    AppropriateCCH,
    Timeslots(bool, bool, bool, bool)
}

impl Decodable for TimeslotAssigned {
    fn decode(cursor: &mut Cursor) -> Self {
        match cursor.read_int(2) {
            0b0000 => Self::AppropriateCCH,
            timeslots @ 0b0001..=0b1111 => Self::Timeslots(
                timeslots & 0b0001 > 0,
                timeslots & 0b0010 > 0,
                timeslots & 0b0100 > 0,
                timeslots & 0b1000 > 0,
            ),
            unknown => panic!("unknown assigned timeslot value {unknown}")
        }
    }
}
