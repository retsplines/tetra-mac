use bitvec::macros::internal::funty::Fundamental;
use bitvec::prelude::*;
use crate::codec::{Cursor, Decodable, Encodable, Optional};
use crate::pdu::downlink::partial::{Address, ChannelAllocation, Length, PowerControl, RandomAccessFlag, SlotGranting};
use crate::pdu::DownlinkMACPDUType;


#[derive(Debug)]
pub struct MACResourcePDU {
    pdu_type: DownlinkMACPDUType,
    fill_bit_indication: bool,
    grant_is_on_current_channel: bool,
    encryption_mode: u32,
    random_access: RandomAccessFlag,
    length: Length,
    address: Address,
    power_control: Optional<PowerControl>,
    slot_granting: Optional<SlotGranting>,
    channel_allocation: Optional<ChannelAllocation>
}

impl Decodable for MACResourcePDU {
    fn decode(cursor: &mut Cursor) -> Self {

        // Decode & validate the PDU type
        let pdu_type = DownlinkMACPDUType::decode(cursor);
        assert_eq!(pdu_type, DownlinkMACPDUType::MACResource);

        MACResourcePDU {
            pdu_type,
            fill_bit_indication: cursor.read_bool(),
            grant_is_on_current_channel: cursor.read_bool(),
            encryption_mode: cursor.read_int(2),
            random_access: RandomAccessFlag::decode(cursor),
            length: Length::decode(cursor),
            address: Address::decode(cursor),
            power_control: Optional::decode(cursor),
            slot_granting: Optional::decode(cursor),
            channel_allocation: Optional::decode(cursor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decodes_correctly() {

        let mut data: Vec<u8> = vec![
            0x20, 0x69, 0x00, 0x04, 0x02, 0x03, 0x48, 0x40,
            0x00, 0x00, 0x4e, 0xab, 0x10, 0x00, 0x10, 0x80
        ];

        // Create a cursor over the data
        let mut cur = Cursor::new(data.as_mut_bits::<Msb0>());

        let pdu = MACResourcePDU::decode(&mut cur);

        // MAC-RESOURCE type PDU
        assert_eq!(pdu.pdu_type, DownlinkMACPDUType::MACResource);

        // Fill bits are present
        assert_eq!(pdu.fill_bit_indication, true);

        // Grant not on current channel (because no granting element)
        assert_eq!(pdu.grant_is_on_current_channel, false);

        println!("{:?}", pdu);

    }

}