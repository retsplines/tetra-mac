use crate::codec::{Reader, Decodable};

#[derive(Debug)]
pub enum GrantingDelay {
    AtNextOpportunity,
    After(u32),
    Frame18,
    WaitForAnotherMessage
}

impl Decodable for GrantingDelay {
    fn decode(reader: &mut Reader) -> Self {
        match reader.read_int(4) {
            0b0000 => Self::AtNextOpportunity,
            after @ 0b0001 ..= 0b1101 => Self::After(after),
            0b1110 => Self::Frame18,
            0b1111 => Self::WaitForAnotherMessage,
            unknown => panic!("unknown slot granting delay {unknown}")
        }
    }
}