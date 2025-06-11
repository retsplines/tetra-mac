use crate::codec::{Builder, Decodable, Encodable, Reader};
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
    pub frame_18_extension: bool
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
        };

        // Read the reserved bit(s)
        reader.read_bool();

        result
    }
}

impl Encodable for Sync {
    fn encode(&self, builder: &mut Builder) {
        builder.write_int(self.system_code, 4);
        builder.write_int(self.colour_code, 6);
        builder.write_int(self.timeslot_number, 2);
        builder.write_int(self.frame_number, 5);
        builder.write_int(self.multiframe_number, 6);
        builder.write_int(num::ToPrimitive::to_u32(&self.sharing_mode).unwrap(), 2);
        builder.write_int(num::ToPrimitive::to_u32(&self.ts_reserved_frames).unwrap(), 3);
        builder.write_bool(self.u_plane_dtx);
        builder.write_bool(self.frame_18_extension);

        // Reserved:
        builder.write_bool(false);
    }
}

mod tests {
    use crate::Bits;
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
        };

        let mut builder = Builder::new();
        pdu.encode(&mut builder);

        // Obtain the bits
        let bits = builder.done();
        dbg!(bits);
    }

    #[test]
    fn decodes() {

        let data = Bits::from_vec(vec![
            0b0000_1111, 0b11_01_0001, 0b1_000111_0, 0b0_001_0_1_0_0
        ]);

        let mut reader = Reader::new(&data);
        let sync_pdu = Sync::decode(&mut reader);
        
        assert_eq!(sync_pdu.system_code, 0);
        assert_eq!(sync_pdu.colour_code, 63);
        assert_eq!(sync_pdu.timeslot_number, 1);
        assert_eq!(sync_pdu.frame_number, 3);
        assert_eq!(sync_pdu.multiframe_number, 7);
        assert_eq!(sync_pdu.sharing_mode, SharingMode::ContinuousTransmission);
        assert_eq!(sync_pdu.ts_reserved_frames, TSReservedFrames::Reserve2);
        assert_eq!(sync_pdu.frame_18_extension, true);

    }

}