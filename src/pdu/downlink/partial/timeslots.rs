use crate::codec::{Builder, Encodable};

type Timeslots = (bool, bool, bool, bool);

impl Encodable for Timeslots {
    fn encode(&self, builder: &mut Builder) {
        todo!()
    }
}