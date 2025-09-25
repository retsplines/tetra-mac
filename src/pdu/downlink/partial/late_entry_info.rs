use crate::codec::{Decodable, Encodable, Reader, Writer};

#[derive(Debug)]
pub struct LateEntryInfo {
    late_entry_supported: bool,
}

impl Decodable for LateEntryInfo {
    fn decode(reader: &mut Reader) -> Self {
        LateEntryInfo {
            late_entry_supported: reader.read_bool()
        }
    }
}

impl Encodable for LateEntryInfo {
    fn encode(&self, writer: &mut Writer) {
        writer.write_bool(self.late_entry_supported);
    }
}
