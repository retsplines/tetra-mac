mod partial;
mod normal_cont_dl_burst;
mod sync_cont_dl_burst;
mod synchroniser;

pub use sync_cont_dl_burst::SyncContDownlinkBurst;
pub use normal_cont_dl_burst::NormalContDownlinkBurst;

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
