use crate::codec::{Decodable, Encodable, Reader, Writer};

#[derive(Debug)]
pub struct BSServiceDetails {
    registration_required: bool,
    deregistration_required: bool,
    priority_cell: bool,
    cell_never_uses_minimum_mode: bool,
    migration: bool,
    system_wide_services: bool,
    tetra_voice_service: bool,
    circuit_mode_data_service: bool,
    sndcp_service: bool,
    air_interface_encryption_service: bool,
    advanced_link_supported: bool
}

impl Decodable for BSServiceDetails {
    fn decode(reader: &mut Reader) -> Self {
        let pdu = BSServiceDetails {
            registration_required: reader.read_bool(),
            deregistration_required: reader.read_bool(),
            priority_cell: reader.read_bool(),
            cell_never_uses_minimum_mode: reader.read_bool(),
            migration: reader.read_bool(),
            system_wide_services: reader.read_bool(),
            tetra_voice_service: reader.read_bool(),
            circuit_mode_data_service: reader.read_bool(),
            sndcp_service: false,
            air_interface_encryption_service: false,
            advanced_link_supported: false,
        };

        // Read the reserved bit
        reader.read_bool();

        // Read the remaining bits in
        BSServiceDetails {
            sndcp_service: reader.read_bool(),
            air_interface_encryption_service: reader.read_bool(),
            advanced_link_supported: reader.read_bool(),
            ..pdu
        }
    }
}

impl Encodable for BSServiceDetails {
    fn encode(&self, writer: &mut Writer) {
        writer.write_bool(self.registration_required);
        writer.write_bool(self.deregistration_required);
        writer.write_bool(self.priority_cell);
        writer.write_bool(self.cell_never_uses_minimum_mode);
        writer.write_bool(self.migration);
        writer.write_bool(self.system_wide_services);
        writer.write_bool(self.tetra_voice_service);
        writer.write_bool(self.circuit_mode_data_service);
        writer.write_bool(false); // reserved
        writer.write_bool(self.sndcp_service);
        writer.write_bool(self.air_interface_encryption_service);
        writer.write_bool(self.advanced_link_supported);
}
}
