use super::reservation_requirement::ReservationRequirement;

#[derive(Debug)]
pub enum LengthOrCapacityRequest {
    Length,
    CapacityRequest {
        fragmentation: bool,
        reservation_requirement: ReservationRequirement
    }
}
