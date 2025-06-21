use crate::codec::{Writer, Decodable, Encodable, Reader};

pub type Timeslots = (bool, bool, bool, bool);

impl Encodable for Timeslots {
    fn encode(&self, writer: &mut Writer) {
        writer.write_int(
            (self.0 as u32) << 3 |
            (self.1 as u32) << 2 |
            (self.2 as u32) << 1 |
            (self.3 as u32)
        , 4);
    }
}

impl Decodable for Timeslots {
    fn decode(reader: &mut Reader) -> Self {
        let bitmap = reader.read_int(4);
        (
            bitmap & 0b0001 > 0,
            bitmap & 0b0010 > 0,
            bitmap & 0b0100 > 0,
            bitmap & 0b1000 > 0,
        )
    }
}
