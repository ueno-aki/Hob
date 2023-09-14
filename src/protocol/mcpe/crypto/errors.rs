use thiserror::Error;

#[derive(Debug,Error)]
pub enum CryptoErrors {
    #[error("Failed Verification:{0:?}")]
    FailedVerification(String)
}