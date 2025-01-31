use crate::codec::{Reader, Decodable, Encodable, Builder};
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
    fn encode(&self, builder: &mut Builder) {
        builder.write_int(match self {
            Self::AppropriateCCH => 0b0000,
            Self::Specific(timeslots) =>
                (timeslots.0 as u32) << 3 |
                (timeslots.1 as u32) << 2 |
                (timeslots.2 as u32) << 1 |
                (timeslots.3 as u32)
        }, 4);
    }
}

mod test {

    use bitvec::{bits};
    use bitvec::order::Msb0;
    use crate::codec::{Builder, Encodable};
    use crate::pdu::downlink::partial::TimeslotAssigned;

    #[test]
    fn it_encodes_correctly() {
        let tsa = TimeslotAssigned::Specific((false, true, false, true));
        let mut builder = Builder::new();
        tsa.encode(&mut builder);
        assert_eq!(builder.done(), bits![u8, Msb0; 0, 1, 0, 1]);
    }
}