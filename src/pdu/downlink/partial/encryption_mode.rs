#[derive(Debug)]
pub enum EncryptionMode {
    NotEncrypted = 0b00,
    EncryptedA = 0b01,
    EncryptedB = 0b10,
    EncryptedC = 0b11
}
