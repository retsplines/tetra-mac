use crate::bits::Bits;
use crate::logical_channels::LogicalChannel;
use crate::codec::{Writer, Encodable, add_fill_bits, FillBitCapacity};
use crate::lower::scrambler::State;
use crate::pdu;
use crate::tdma_time::TDMATime;
use crate::pdu::downlink::*;

/// A request from the upper MAC to transmit a MAC block using the specified channel coding
pub(crate) struct TMVUnitData {
    pub(crate) primary: TMVUnitDataChannel,
    pub(crate) secondary: Option<TMVUnitDataChannel>,
    pub(crate) aach: Bits,
}

pub(crate) struct TMVUnitDataChannel {
    pub(crate) mac_block: Bits,
    pub(crate) logical_channel: LogicalChannel,
    pub(crate) scrambling_code: State
}

/// The state of the Upper MAC
pub struct UpperMAC { }

impl UpperMAC {

    pub fn new() -> Self {
        UpperMAC {}
    }

    /// Should the BNCH be mapped?
    fn slot_should_be_bnch(&self, time: &TDMATime) -> bool {
        // During the control frame, BNCH appears if (MN + TN) % 4 == 1
        time.is_control_frame() && (time.multiframe() + time.slot()) % 4 == 1
    }

    /// Should the BSCH be mapped?
    fn slot_should_be_bsch(&self, time: &TDMATime) -> bool {
        // During the control frame, BSCH appears if (MN + TN) % 4 = 3
        time.is_control_frame() && (time.multiframe() + time.slot()) % 4 == 3
    }

    /// Generate a half-slot with no content
    fn generate_null_sch_hd(&self) -> Bits {

        let null_pdu = pdu::downlink::MACResourcePDU::null();
        let mut writer = Writer::new();
        null_pdu.encode(&mut writer);
        let mut null_pdu_bits = writer.done();

        // Fill bits
        add_fill_bits(&mut null_pdu_bits, FillBitCapacity::Bits(124));

        null_pdu_bits
    }

    pub fn generate_slot(&self, time: &TDMATime) -> TMVUnitData {

        // Broadcast Network Channel mapped in this slot?
        if self.slot_should_be_bnch(time) {
            let bnch_bits = self.generate_bnch();
            return TMVUnitData {
                primary: TMVUnitDataChannel {
                    mac_block: bnch_bits,
                    logical_channel: LogicalChannel::BroadcastNetwork,
                    scrambling_code: State::new(234, 0, 0), // todo: configuration?
                },
                secondary: Some(TMVUnitDataChannel {
                    mac_block: self.generate_null_sch_hd(),
                    logical_channel: LogicalChannel::SignallingHalfDownlink,
                    scrambling_code: State::zero()
                }),
                aach: Default::default(),
            }
        }

        // Broadcast Sync Channel mapped in this slot?
        if self.slot_should_be_bsch(time) {
            let bsch_bits = self.generate_bsch();
            return TMVUnitData {
                primary: TMVUnitDataChannel {
                    mac_block: bsch_bits,
                    logical_channel: LogicalChannel::BroadcastSynchronisation,
                    scrambling_code: State::zero()
                },
                secondary: Some(TMVUnitDataChannel {
                    mac_block: self.generate_null_sch_hd(),
                    logical_channel: LogicalChannel::SignallingHalfDownlink,
                    scrambling_code: State::zero()
                }),
                aach: Default::default(),
            }
        }

        // Placeholder: generate two empty half-slots
        // This behaviour is specified for frames with no signalling
        let null_sch_hd_bits = self.generate_null_sch_hd();
        TMVUnitData {
            primary: TMVUnitDataChannel {
                mac_block: null_sch_hd_bits.clone(),
                logical_channel: LogicalChannel::SignallingHalfDownlink,
                scrambling_code: State::new(234, 0, 0)
            },
            secondary: Some(TMVUnitDataChannel {
                mac_block: null_sch_hd_bits.clone(),
                logical_channel: LogicalChannel::SignallingHalfDownlink,
                scrambling_code: State::new(234, 0, 0)
            }),
            aach: Default::default()
        }
    }

    /// Generate the BNCH
    /// todo: This will be outsourced to a helper that maintains state for the optional fields,
    /// todo: accesses configuration etc.
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
        let mut bnch_bits = writer.done();

        // Fill bits
        add_fill_bits(&mut bnch_bits, FillBitCapacity::Bits(124));

        bnch_bits
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
        writer.done()
    }
}

