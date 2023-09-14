use thiserror::Error;

#[derive(Debug,Error)]
pub enum CryptoErrors {
    #[error("InvalidJWTFormat:{0}")]
    InvalidJWTFormat(String),
    #[error("InvalidJWTFormat:{0}")]
    InvalidJWTPayload(String),
    #[error("Failed Verification")]
    FailedVerification
}