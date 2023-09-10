use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransFormError {
    #[error("Unspecified Packet:{0:?}")]
    ClientUnspecifiedPacket(Vec<u8>),
}
