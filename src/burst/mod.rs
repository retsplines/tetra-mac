mod partial;
mod normal_cont_dl_burst;
mod sync_cont_dl_burst;

#[derive(Debug)]
pub enum BurstExtractionError {
    IncorrectLength {
        expected: usize,
        provided: usize
    },
    InvalidSequence
}
