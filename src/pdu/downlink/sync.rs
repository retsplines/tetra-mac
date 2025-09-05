use crate::bits::Bits;
use crate::codec::{Writer, Decodable, Encodable, Reader};
use crate::pdu::downlink::partial::{SharingMode, TSReservedFrames};

#[derive(Debug)]
pub struct Sync {
    pub system_code: u32,
    pub colour_code: u32,
    pub timeslot_number: u32,
    pub frame_number: u32,
    pub multiframe_number: u32,
    pub sharing_mode: SharingMode,
    pub ts_reserved_frames: TSReservedFrames,
    pub u_plane_dtx: bool,
    pub frame_18_extension: bool,
    pub tm_sdu_bits: Bits
}

impl Decodable for Sync {
    fn decode(reader: &mut Reader) -> Self {

        let result = Sync {
            system_code: reader.read_int(4),
            colour_code: reader.read_int(6),
            timeslot_number: reader.read_int(2),
            frame_number: reader.read_int(5),
            multiframe_number: reader.read_int(6),
            sharing_mode: num::FromPrimitive::from_u32(reader.read_int(2)).unwrap(),
            ts_reserved_frames: num::FromPrimitive::from_u32(reader.read_int(3)).unwrap(),
            u_plane_dtx: reader.read_bool(),
            frame_18_extension: reader.read_bool(),
            tm_sdu_bits: Bits::new()
        };

        // Consume the reserved bit(s)
        reader.read_bool();

        // Read the TM-SDU bits
        let bits = reader.read(29);

        Sync {
            tm_sdu_bits: bits,
            ..result
        }
    }
}

impl Encodable for Sync {
    fn encode(&self, writer: &mut Writer) {

        writer.write_int(self.system_code, 4);
        writer.write_int(self.colour_code, 6);
        writer.write_int(self.timeslot_number, 2);
        writer.write_int(self.frame_number, 5);
        writer.write_int(self.multiframe_number, 6);
        writer.write_int(num::ToPrimitive::to_u32(&self.sharing_mode).unwrap(), 2);
        writer.write_int(num::ToPrimitive::to_u32(&self.ts_reserved_frames).unwrap(), 3);
        writer.write_bool(self.u_plane_dtx);
        writer.write_bool(self.frame_18_extension);

        // Reserved:
        writer.write_bool(false);
        
        // Write the TM-SDU bits
        writer.write(&self.tm_sdu_bits);
    }
}

mod tests {
    use bitvec::prelude::*;
    use crate::new_bits;
    use super::*;

    #[test]
    fn encodes() {
        let pdu = Sync {
            system_code: 0,
            colour_code: 32,
            timeslot_number: 1,
            frame_number: 2,
            multiframe_number: 3,
            sharing_mode: SharingMode::ContinuousTransmission,
            ts_reserved_frames: TSReservedFrames::Reserve6,
            u_plane_dtx: false,
            frame_18_extension: true,
            tm_sdu_bits: Bits::repeat(false, 29)
        };

        let mut writer = Writer::new();
        pdu.encode(&mut writer);

        // Obtain the bits
        let bits = writer.done();
        dbg!(bits);
    }

    #[test]
    fn decodes() {

        let data = new_bits![
            0, 0, 0, 0, // system code
            1, 1, 1, 1, 1, 1, // colour code
            0, 1, // timeslot number
            0, 0, 0, 1, 1, // frame number
            0, 0, 0, 1, 1, 1, // multiframe number
            0, 0, // sharing mode (ContinuousTransmission)
            0, 0, 1, // ts_reserved_frames (Reserve2)
            0, // u_plane_dtx
            1, // frame_18_extension
            0, // reserved bit
            0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0 // tm_sdu_bits (29 bits)
        ];

        let mut reader = Reader::new(&data);
        let sync_pdu = Sync::decode(&mut reader);

        assert_eq!(sync_pdu.system_code, 0);
        assert_eq!(sync_pdu.colour_code, 63);
        assert_eq!(sync_pdu.timeslot_number, 1);
        assert_eq!(sync_pdu.frame_number, 3);
        assert_eq!(sync_pdu.multiframe_number, 7);
        assert_eq!(sync_pdu.sharing_mode, SharingMode::ContinuousTransmission);
        assert_eq!(sync_pdu.ts_reserved_frames, TSReservedFrames::Reserve2);
        assert!(!sync_pdu.u_plane_dtx);
        assert!(sync_pdu.frame_18_extension);
        assert_eq!(sync_pdu.tm_sdu_bits, new_bits![
             0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0
        ]);

    }

}