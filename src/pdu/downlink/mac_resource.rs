use crate::codec::{Reader, Decodable, Encodable, Optional, Builder};
use crate::pdu::downlink::partial::{Address, ChannelAllocation, Length, PowerControl, SlotGranting};
use crate::pdu::DownlinkMACPDUType;


#[derive(Debug)]
pub struct MACResourcePDU {
    pub fill_bit_indication: bool,
    pub grant_is_on_current_channel: bool,
    pub encryption_mode: u32,
    pub random_access_acknowledged: bool,
    pub length: Length,
    pub address: Address,
    pub power_control: Optional<PowerControl>,
    pub slot_granting: Optional<SlotGranting>,
    pub channel_allocation: Optional<ChannelAllocation>
}

impl MACResourcePDU {
    pub fn null() -> Self {
        MACResourcePDU {
            fill_bit_indication: false,
            grant_is_on_current_channel: false,
            encryption_mode: 0,
            random_access_acknowledged: false,
            length: Length::Reserved,
            address: Address::NullPDU,
            power_control: Optional::Absent,
            slot_granting: Optional::Absent,
            channel_allocation: Optional::Absent,
        }
    }
}

impl Decodable for MACResourcePDU {
    fn decode(reader: &mut Reader) -> Self {

        // Decode & validate the PDU type
        let pdu_type = DownlinkMACPDUType::decode(reader);
        assert_eq!(pdu_type, DownlinkMACPDUType::MACResource);

        MACResourcePDU {
            fill_bit_indication: reader.read_bool(),
            grant_is_on_current_channel: reader.read_bool(),
            encryption_mode: reader.read_int(2),
            random_access_acknowledged: reader.read_bool(),
            length: Length::decode(reader),
            address: Address::decode(reader),
            power_control: Optional::decode(reader),
            slot_granting: Optional::decode(reader),
            channel_allocation: Optional::decode(reader)
        }
    }
}

impl Encodable for MACResourcePDU {
    fn encode(&self, builder: &mut Builder) {
        DownlinkMACPDUType::MACResource.encode(builder);
        builder.write_bool(self.fill_bit_indication);
        builder.write_bool(self.grant_is_on_current_channel);
        builder.write_int(self.encryption_mode, 2);
        builder.write_bool(self.random_access_acknowledged);
        self.length.encode(builder);
        self.address.encode(builder);
        self.power_control.encode(builder);
        self.slot_granting.encode(builder);
        self.channel_allocation.encode(builder);
    }
}

#[cfg(test)]
mod tests {

    use bitvec::prelude::*;
    use super::*;
    use crate::Bits;

    #[test]
    fn decodes() {

        let data = Bits::from_vec(vec![
            0x20, 0x69, 0x00, 0x04, 0x02, 0x03, 0x48, 0x40,
            0x00, 0x00, 0x4e, 0xab, 0x10, 0x00, 0x10, 0x80
        ]);

        // Create a reader over the data
        let mut cur = Reader::new(&data);

        let pdu = MACResourcePDU::decode(&mut cur);

        // Fill bits are present
        assert_eq!(pdu.fill_bit_indication, true);

        // Grant not on current channel (because no granting element)
        assert_eq!(pdu.grant_is_on_current_channel, false);
    }

    #[test]
    fn encodes() {

        let mac_resource = MACResourcePDU {
            fill_bit_indication: true,
            grant_is_on_current_channel: false,
            encryption_mode: 0,
            random_access_acknowledged: false,
            length: Length::Octets(32),
            address: Address::SSI {
                address: 1026,
            },
            power_control: Optional::Absent,
            slot_granting: Optional::Absent,
            channel_allocation: Optional::Absent,
        };

        let mut builder = Builder::new();
        mac_resource.encode(&mut builder);
        let bits = builder.done();

        assert_eq!(bits, bits![u8, Msb0;
            0,0, 1, 0, 0,0, 0, 1,0,0,0,0,0, 0,0,1, 0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0, 0, 0, 0
        ]);

    }

}