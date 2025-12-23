use crate::burst::{NormalContDownlinkBurst, SyncContDownlinkBurst, DownlinkBurst};
use crate::logical_channels::LogicalChannel;
use crate::tdma_time::TDMATime;
use crate::upper_mac::UpperMAC;

/// Generate a downlink slot for the provided time.
/// Requests MAC blocks from the upper MAC and maps them onto a burst ready for the physical layer
pub(crate) fn generate_dl_slot(time: &TDMATime) -> DownlinkBurst {

    // TODO: this will be passed-in?
    let mac = UpperMAC::new();

    // Request the block(s)
    let blocks = mac.generate_slot(time);

    // Burst type shall be based on the channel of the primary block
    match blocks.primary.logical_channel {

        // BSCH + (SCH/HD or BNCH) => SB
        LogicalChannel::BroadcastSynchronisation => {

            DownlinkBurst::Sync (SyncContDownlinkBurst {
                sb1_bits: blocks.primary.logical_channel.encode(
                    blocks.primary.mac_block,
                    &blocks.primary.scrambling_code
                ),
                sb2_bits: match blocks.secondary {
                    Some(block) => block.logical_channel.encode(
                        block.mac_block,
                        &block.scrambling_code
                    ),
                    None => panic!("BSCH provided without SB2 content")
                },
                bb_bits: Default::default(),
            })

        }

        // SCH/HD + SCH/HD => NDB
        LogicalChannel::SignallingHalfDownlink => {

            DownlinkBurst::Normal (NormalContDownlinkBurst {
                bkn1_bits: blocks.primary.logical_channel.encode(
                    blocks.primary.mac_block,
                    &blocks.primary.scrambling_code
                ),
                bkn2_bits: match blocks.secondary {
                    Some(block) => block.logical_channel.encode(
                        block.mac_block,
                        &block.scrambling_code
                    ),
                    None => panic!("BSCH provided without SB2 content")
                },
                bb_bits: Default::default(),
                slot_flag: false
            })

        },

        // BNCH + SCH/HD => NDB
        LogicalChannel::BroadcastNetwork => {

            // BNCH is always mapped to bkn2, so send the secondary block in bkn1
            DownlinkBurst::Normal (NormalContDownlinkBurst {
                bkn1_bits: match blocks.secondary {
                    Some(block) => block.logical_channel.encode(
                        block.mac_block,
                        &block.scrambling_code
                    ),
                    None => panic!("BNCH provided without SB1 content")
                },
                bkn2_bits: blocks.primary.logical_channel.encode(
                    blocks.primary.mac_block,
                    &blocks.primary.scrambling_code
                ),
                bb_bits: Default::default(),
                slot_flag: false
            })

        },

        // STCH + SCH/HD => NDB + SF
        LogicalChannel::Stealing => { todo!("downlink stealing not implemented") }

        // SCH/F => NDB
        LogicalChannel::SignallingFull => {

            // Not possible to multiplex, so secondary should be None
            assert!(blocks.secondary.is_none(), "Multiplexing requested but SCH/F provided");

            // Split the channel bits between the two burst blocks
            let burst_bits = blocks.primary.logical_channel.encode(
                blocks.primary.mac_block,
                &blocks.primary.scrambling_code
            );

            DownlinkBurst::Normal (NormalContDownlinkBurst {
                bkn1_bits: burst_bits[..216].to_bitvec(),
                bkn2_bits: burst_bits[216..].to_bitvec(),
                bb_bits: Default::default(),
                slot_flag: false
            })

        }
        _ => panic!("invalid primary block type")
    }



}