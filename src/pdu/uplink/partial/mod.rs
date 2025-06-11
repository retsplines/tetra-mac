/// PDU partials used within PDUs

pub use address::Address;
pub use length_or_capacity_request::LengthOrCapacityRequest;

mod length_or_capacity_request;
mod address;
mod reservation_requirement;
