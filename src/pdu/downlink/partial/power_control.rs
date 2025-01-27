use crate::codec::{Reader, Decodable, Encodable, Builder};

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
    fn decode(reader: &mut Reader) -> Self {
        match reader.read_int(4) {
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

impl Encodable for PowerControl {
    fn encode(&self, builder: &mut Builder) {
        builder.write_int(match self {
            Self::NoChange => 0b0000,
            PowerControl::IncreaseBySteps(inc) => *inc,
            PowerControl::MaximumPathDelayExceeded => 0b0111,
            PowerControl::OpenLoop => 0b1000,
            PowerControl::DecreaseBySteps(dec) => (*dec) + 8,
            PowerControl::RadioUplinkFailure => 0b1111
        }, 4);
    }
}