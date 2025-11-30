use crate::codec::{Reader, Decodable, Encodable, Writer};
use crate::pdu::downlink::partial::Timeslots;

#[derive(Debug)]
pub enum TimeslotAssigned {
    AppropriateCCH,
    Specific(Timeslots)
}

impl Decodable for TimeslotAssigned {
    fn decode(reader: &mut Reader) -> Self {
        match reader.read_int(2) {
            0b0000 => Self::AppropriateCCH,
            timeslots @ 0b0001..=0b1111 => Self::Specific((
                timeslots & 0b0001 > 0,
                timeslots & 0b0010 > 0,
                timeslots & 0b0100 > 0,
                timeslots & 0b1000 > 0,
            )),
            unknown => panic!("unknown assigned timeslot value {unknown}")
        }
    }
}

impl Encodable for TimeslotAssigned {
    fn encode(&self, writer: &mut Writer) {
        writer.write_int(match self {
            Self::AppropriateCCH => 0b0000,
            Self::Specific(timeslots) =>
                (timeslots.0 as u32) << 3 |
                (timeslots.1 as u32) << 2 |
                (timeslots.2 as u32) << 1 |
                (timeslots.3 as u32)
        }, 4);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::bits::from_bitstr;
    use crate::codec::{Writer, Encodable};

    #[test]
    fn encodes() {
        let tsa = TimeslotAssigned::Specific((false, true, false, true));
        let mut writer = Writer::new();
        tsa.encode(&mut writer);
        assert_eq!(writer.done(), from_bitstr("0101"));
    }
}