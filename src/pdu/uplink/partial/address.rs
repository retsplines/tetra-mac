#[derive(Debug)]
pub enum Address {
    SSI { ssi: u32 },
    USSI { ussi: u32 },
    SMI {ssi: u32 } ,
    EventLabel { event_label: u32 }
}
