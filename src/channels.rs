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
    pub fn build(logical_channel: LogicalChannel) -> ChannelProperties {
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
    pub fn encode(self, type1_bits: Bits) -> Bits {

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
            // TODO use the correct scrambler state, which means this needs to be aware of MNC/MCC/BCC
            // TODO should the scrambler state be passed-in maybe?
            let mut scrambler_state = State::new(0, 0, 0);
            scrambler_encode(&type4_bits, &mut scrambler_state)
        } else {
            type4_bits
        }
    }

    /// Decode this channel from bits, applying the appropriate decoding chain
    pub fn decode(self, type5_bits: Bits) -> Bits {

        // Resolve channel props
        let chan_props = ChannelProperties::build(self);

        // Scrambling?
        let type4_bits = if chan_props.scrambling {
            // TODO use the correct scrambler state, which means this needs to be aware of MNC/MCC/BCC
            // TODO should the scrambler state be passed-in maybe?
            let mut scrambler_state = State::new(0, 0, 0);
            scrambler_decode(&type5_bits, &mut scrambler_state)
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

    #[test]
    fn decodes_channel_correctly() {
        
        
        
    }

}