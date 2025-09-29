use crate::bits::Bits;
use crate::lower::rcpc::puncturers::{PredefinedPuncturer, Puncturer};
use crate::lower::block_coder::{block_decode, block_encode};
use crate::lower::rm_coder::{rm_decode, rm_encode};
use crate::lower::rcpc::{rcpc_decode, rcpc_encode};
use crate::lower::interleaver::{interleaver_decode, interleaver_encode};
use crate::lower::scrambler::{scrambler_decode, scrambler_encode, State};

pub enum InitialCode {
    RMCode,
    BlockCode
}

pub enum InterleaverBehaviour {
    Block { k: usize, a: usize },
    OverNBlocks
}

pub struct ChannelProperties {
    initial_code: Option<InitialCode>,
    rcpc: Option<PredefinedPuncturer>,
    tail_bits: usize,
    interleaver: Option<InterleaverBehaviour>,
    scrambling: bool,
}

pub enum LogicalChannel {
    AccessAssignment,
    BroadcastSynchronisation,
    TrafficHighProtection,
    TrafficLowProtection,
    TrafficUnprotected,
    SignallingHalfDownlink,
    BroadcastNetwork,
    Stealing,
    SignallingHalfUplink,
    SignallingFull
}

impl ChannelProperties {
    pub fn build(logical_channel: &LogicalChannel) -> ChannelProperties {
        match logical_channel {
            LogicalChannel::AccessAssignment => ChannelProperties {
                initial_code: Some(InitialCode::RMCode),
                rcpc: None,
                tail_bits: 0,
                interleaver: None,
                scrambling: true,
            },
            LogicalChannel::BroadcastSynchronisation => ChannelProperties {
                initial_code: Some(InitialCode::BlockCode),
                rcpc: Some(PredefinedPuncturer::Rate2Over3Puncturer),
                tail_bits: 4,
                interleaver: Some(InterleaverBehaviour::Block { k: 120, a: 11 }),
                scrambling: true,
            },
            LogicalChannel::TrafficHighProtection => ChannelProperties {
                initial_code: None,
                rcpc: Some(PredefinedPuncturer::Rate148Over432Puncturer),
                tail_bits: 4,
                interleaver: Some(InterleaverBehaviour::OverNBlocks),
                scrambling: true,
            },
            LogicalChannel::TrafficLowProtection => ChannelProperties {
                initial_code: None,
                rcpc: Some(PredefinedPuncturer::Rate292Over432Puncturer),
                tail_bits: 4,
                interleaver: Some(InterleaverBehaviour::OverNBlocks),
                scrambling: true,
            },
            LogicalChannel::TrafficUnprotected => ChannelProperties {
                initial_code: None,
                rcpc: None,
                tail_bits: 0,
                interleaver: None,
                scrambling: true,
            },
            LogicalChannel::SignallingHalfDownlink |
            LogicalChannel::BroadcastNetwork |
            LogicalChannel::Stealing => ChannelProperties {
                initial_code: Some(InitialCode::BlockCode),
                rcpc: Some(PredefinedPuncturer::Rate2Over3Puncturer),
                tail_bits: 4,
                interleaver: Some(InterleaverBehaviour::Block { k: 216, a: 101 }),
                scrambling: true,
            },
            LogicalChannel::SignallingHalfUplink => ChannelProperties {
                initial_code: Some(InitialCode::BlockCode),
                rcpc: Some(PredefinedPuncturer::Rate2Over3Puncturer),
                tail_bits: 4,
                interleaver: Some(InterleaverBehaviour::Block { k: 168, a: 13 }),
                scrambling: true,
            },
            LogicalChannel::SignallingFull => ChannelProperties {
                initial_code: Some(InitialCode::BlockCode),
                rcpc: Some(PredefinedPuncturer::Rate2Over3Puncturer),
                tail_bits: 4,
                interleaver: Some(InterleaverBehaviour::Block { k: 432, a: 103 }),
                scrambling: true,
            }
        }
    }
}

impl LogicalChannel {

    /// Generate this channel from bits, applying the appropriate encoding chain
    pub fn encode(&self, type1_bits: Bits, scrambler_state: &State) -> Bits {

        // Resolve channel props
        let chan_props = ChannelProperties::build(self);

        // Initial coding
        let mut type2_bits = match chan_props.initial_code {
            Some(InitialCode::RMCode) => rm_encode(&type1_bits).unwrap(),
            Some(InitialCode::BlockCode) => block_encode(&type1_bits),
            None => type1_bits
        };

        // Tail bits?
        if chan_props.tail_bits != 0 {
            type2_bits.extend([false; 4]);
        }

        // RCPC?
        let type3_bits = match chan_props.rcpc {
            Some(predefined_punc) =>
                rcpc_encode(&type2_bits, Some(&Puncturer::build(&predefined_punc))),
            None => type2_bits
        };

        // Interleaving?
        let type4_bits = match chan_props.interleaver {
            Some(InterleaverBehaviour::Block {k, a}) => interleaver_encode(&type3_bits, k, a).unwrap(),
            Some(InterleaverBehaviour::OverNBlocks) => todo!("over-N-blocks interleaving not yet supported"),
            None => type3_bits
        };

        // Scrambling?
        if chan_props.scrambling {
            scrambler_encode(&type4_bits, scrambler_state)
        } else {
            type4_bits
        }
    }

    /// Decode this channel from bits, applying the appropriate decoding chain
    pub fn decode(&self, type5_bits: Bits, scrambler_state: &State) -> Bits {

        // Resolve channel props
        let chan_props = ChannelProperties::build(self);

        // Scrambling?
        let type4_bits = if chan_props.scrambling {
            scrambler_decode(&type5_bits, scrambler_state)
        } else {
            // No scrambling applied
            type5_bits
        };

        // Interleaving?
        let type3_bits = match chan_props.interleaver {
            Some(InterleaverBehaviour::Block {k, a}) => interleaver_decode(&type4_bits, k, a),
            Some(InterleaverBehaviour::OverNBlocks) => todo!("over-N-blocks interleaving not yet supported"),
            None => type4_bits
        };

        // RCPC?
        let mut type2_bits = match chan_props.rcpc {
            Some(predefined_punc) =>
                rcpc_decode(&type3_bits, Some(&Puncturer::build(&predefined_punc))),
            None => type3_bits
        };

        // Strip tail bits?
        if chan_props.tail_bits != 0 {
            type2_bits.truncate(type2_bits.len() - chan_props.tail_bits);
        }

        // Initial coding?
        match chan_props.initial_code {
            Some(InitialCode::RMCode) => rm_decode(&type2_bits).unwrap(),
            Some(InitialCode::BlockCode) => block_decode(&type2_bits).unwrap(),
            None => type2_bits
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::new_bits;
    use bitvec::prelude::*;
    use crate::bits::{Bits};
    use crate::channels::LogicalChannel;
    use crate::pdu::downlink::{MLESyncPDU, Sync, Sysinfo};
    use crate::codec::{Decodable, Reader};
    use crate::lower::scrambler::State;

    #[test]
    fn sync_channel_encode_decode_consistent() {

        let scrambler_state = State::new(0, 0, 0);
        let mut pattern = Bits::repeat(false, 60);
        pattern.fill_with(|idx| idx % 2 == 0);
        assert_eq!(pattern.len(), 60);
        println!("Pattern {:?}", pattern);

        let lch = LogicalChannel::BroadcastSynchronisation;
        let encoded_bits = lch.encode(pattern.clone(), &scrambler_state);
        assert_eq!(encoded_bits.len(), 120);
        println!("Encoded {:?}", encoded_bits);

        // Decode the channel
        let decoded_bits = lch.decode(encoded_bits, &scrambler_state);
        println!("Decoded {:?}", decoded_bits);

        // Should be the same
        assert_eq!(decoded_bits, pattern);
    }

    #[test]
    fn decodes_bsch_channel_correctly() {
        
        let chan_bits = new_bits![
            0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1,
        ];

        let scrambler_state = State::new(0, 0, 0);

        // Try decoding
        let lch = LogicalChannel::BroadcastSynchronisation;
        let decoded_bits = lch.decode(chan_bits, &scrambler_state);

        // This will be removed or move to integration tests later, but try decoding the sync PDU
        let mut reader = Reader::new(&decoded_bits);
        let sync = Sync::decode(&mut reader);
        println!("{:?}", sync);

        // Decode the MLE sync info
        let mut reader = Reader::new(&sync.tm_sdu_bits);
        let mle_sync = MLESyncPDU::decode(&mut reader);
        println!("{:?}", mle_sync);
    }
}