use super::partial::*;

#[derive(Debug)]
struct MACAccess {
    fill_bit_indication: bool,
    encrypted: bool,
    address: Address,
    length_or_capacity_request: Option<LengthOrCapacityRequest>,
    tm_sdu: Option<Vec<u8>>
}
