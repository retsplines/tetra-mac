use crate::bits::Bits;
use crate::channels::LogicalChannel;
use crate::codec::{Writer, Encodable};
use crate::tdma_time::TDMATime;
use crate::pdu::downlink::*;

/// The possible types of downlink bursts
enum DownlinkBurst {
    Normal {
        block_1: Bits,
        block_2: Bits,
        broadcast: Bits,
        slot_flag: bool
    },
    Synchronisation {
        block_1: Bits,
        block_2: Bits,
        broadcast: Bits,
    }
}

struct MAC {
    tdma: TDMATime
}

impl MAC {
    pub fn new() -> Self {
        MAC {
            tdma: TDMATime::new()
        }
    }

    fn slot_should_be_bnch(&self) -> bool {
        // During the control frame, BNCH appears if (MN + TN) % 4 == 1
        self.tdma.is_control_frame() && (self.tdma.multiframe() + self.tdma.slot()) % 4 == 1
    }

    fn slot_should_be_bsch(&self) -> bool {
        // During the control frame, BSCH appears if (MN + TN) % 4 = 3
        self.tdma.is_control_frame() && (self.tdma.multiframe() + self.tdma.slot()) % 4 == 3
    }

    fn generate_null_sch_hd(&self) -> Bits {

        // let null_pdu = MACResourcePDU::null();
        
        todo!()

    }

    /// Generate a downlink burst for the next slot.
    pub fn generate_next(&self) -> DownlinkBurst {

        // MAC PDUs that we'll prep for this slot
        // Multiple PDUs may be generated and sent together in a slot (called "association")
        // let mut pdus: Vec<Bits> = Vec::new();

        // Broadcast Network Channel mapped in this slot?
        if self.slot_should_be_bnch() {
            
            let bnch_bits = self.generate_bnch();
            
            return DownlinkBurst::Normal {
                block_1: Bits::from_bitslice(&bnch_bits[0..40]),
                block_2: Bits::from_bitslice(&bnch_bits[40..80]),
                broadcast: Bits::new(), // TODO: Access assignment helper
                slot_flag: false
            }
        }

        // BSCH?
        if self.slot_should_be_bsch() {

            let bsch_bits = self.generate_bsch();

            return DownlinkBurst::Synchronisation {
                block_1: Bits::from_bitslice(&bsch_bits[0..40]),
                block_2: Bits::from_bitslice(&bsch_bits[40..80]),
                broadcast: Bits::new(), // TODO: Access assignment helper
            }

        }

        // Generate a null SCH/HD
        let null_sch_hd_bits = self.generate_null_sch_hd();

        DownlinkBurst::Normal {
            block_1: Bits::from_bitslice(&null_sch_hd_bits[0..40]),
            block_2: Bits::from_bitslice(&null_sch_hd_bits[40..80]),
            broadcast: Bits::new(), // TODO: Access assignment helper
            slot_flag: false
        }
    }

    fn generate_bnch(&self) -> Bits {

        let mut writer = Writer::new();

        let sysinfo_pdu = Sysinfo {
            main_carrier: 0,
            frequency_band: 0,
            offset: Offset::NoOffset,
            duplex_spacing: 0,
            reverse: false,
            number_of_common_scch: NumberOfCommonSCCH::None,
            rf_parameters: RFParameters {
                ms_txpwr_max_cell: 0,
                rxlev_access_min: 0,
                access_parameter: 0,
                radio_downlink_timeout: 0
            },
            hyperframe_or_cipher_key: HyperframeOrCipherKey::Hyperframe {
                hyperframe_number: 0
            },
            optional_field: OptionalField::DefaultAccessCodeA(AccessCodeDefinition {
                immediate: Immediate::AlwaysRandomise,
                waiting_time_opportunities: 0,
                number_of_attempts: 0,
                frame_length_x4: false,
                timeslot: TimeslotPointer::SameAsDownlink,
                minimum_priority: 0,
            }),
            tm_sdu_bits: Bits::repeat(false, 42)
        };

        sysinfo_pdu.encode(&mut writer);
        writer.done()
    }

    fn generate_bsch(&self) -> Bits {

        let mut writer = Writer::new();

        let sync_pdu = Sync {
            system_code: 0,
            colour_code: 0,
            timeslot_number: 0,
            frame_number: 0,
            multiframe_number: 0,
            sharing_mode: SharingMode::ContinuousTransmission,
            ts_reserved_frames: TSReservedFrames::Reserve1,
            u_plane_dtx: false,
            frame_18_extension: false,
            tm_sdu_bits: Bits::repeat(false, 29)
        };

        sync_pdu.encode(&mut writer);
        let type1_bits = writer.done();

        let bsch = LogicalChannel::BroadcastSynchronisation;
        bsch.encode(type1_bits)

    }
}

