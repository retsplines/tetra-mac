use crate::codec::{Reader, Decodable, Encodable, Optional, Writer};
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
    fn encode(&self, writer: &mut Writer) {
        DownlinkMACPDUType::MACResource.encode(writer);
        writer.write_bool(self.fill_bit_indication);
        writer.write_bool(self.grant_is_on_current_channel);
        writer.write_int(self.encryption_mode, 2);
        writer.write_bool(self.random_access_acknowledged);
        self.length.encode(writer);
        self.address.encode(writer);
        self.power_control.encode(writer);
        self.slot_granting.encode(writer);
        self.channel_allocation.encode(writer);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::bits::from_bitstr;

    #[test]
    fn decodes() {

        let data = from_bitstr("
            00 // PDU type
            1 // fill bit indication
            0 // position of grant
            00 // encryption mode
            0 // random access
            001101 // length (thirteen octets)
            001 // address type (SSI)
            000000000000010000000010 // address (ten twenty six)
            0 // power control (absent)
            0 // slot granting (no grant)
            0 // channel allocation (none)

            // tm-sdu (77 bits)
            00011010010000100000
            00000000000000000010
            01110101010110001000
            00000000000010000
        ");

        // Create a reader over the data
        let mut cur = Reader::new(&data);

        let pdu = MACResourcePDU::decode(&mut cur);

        // Fill bits are present
        assert_eq!(pdu.fill_bit_indication, true);

        // Grant not on current channel (because no granting element)
        assert_eq!(pdu.grant_is_on_current_channel, false);

        // Length
        assert_eq!(pdu.length, Length::Octets(13));

        // Address
        assert_eq!(pdu.address, Address::SSI { address: 1026 });

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

        let mut writer = Writer::new();
        mac_resource.encode(&mut writer);
        let bits = writer.done();

        assert_eq!(bits, from_bitstr("
            0010000100000001000000000000010000000010000
        "));

    }

}