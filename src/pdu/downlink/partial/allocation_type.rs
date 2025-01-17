use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive)]
pub enum AllocationType {
    Replacement,
    Addition,
    QuitAndGoTo,
    ReplacePlus
}
