mod partial;
mod normal_cont_dl_burst;
mod sync_cont_dl_burst;
mod synchroniser;

pub use sync_cont_dl_burst::SyncContDownlinkBurst;
pub use normal_cont_dl_burst::NormalContDownlinkBurst;
use crate::bits::Bits;

/// A burst that is extractable from Bits
pub trait Extract: Sized {
    fn extract(burst: Bits) -> Result<Self, BurstExtractionError>;
}

/// A burst that is buildable into Bits
pub trait Build {
    fn build(&self) -> Bits;
}

#[derive(Debug)]
pub enum BurstExtractionError {
    IncorrectLength {
        expected: usize,
        provided: usize
    },
    InvalidSequence
}

#[derive(Debug)]
pub enum DownlinkBurst {
    Sync(SyncContDownlinkBurst),
    Normal(NormalContDownlinkBurst),
}
