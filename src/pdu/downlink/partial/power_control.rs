use crate::codec::{Cursor, Decodable};

#[derive(Debug)]
pub enum PowerControl {
    NoChange,
    IncreaseBySteps(u32),
    MaximumPathDelayExceeded,
    OpenLoop,
    DecreaseBySteps(u32),
    RadioUplinkFailure
}

impl Decodable for PowerControl {
    fn decode(cursor: &mut Cursor) -> Self {
        match cursor.read_int(4) {
            0b0000 => PowerControl::NoChange,
            inc @ 0b0001 ..= 0b0110 => PowerControl::IncreaseBySteps(inc),
            0b0111 => PowerControl::MaximumPathDelayExceeded,
            0b1000 => PowerControl::OpenLoop,
            dec @ 0b1001 ..= 0b1110 => PowerControl::DecreaseBySteps(dec - 8),
            0b1111 => PowerControl::RadioUplinkFailure,
            unknown @ _ => panic!("unknown power control information {unknown}")
        }
    }
}
