use thiserror::Error;

#[derive(Debug,Error)]
pub enum LoginErrors {
    #[error("Invalid chains length. found:{0},expected:3")]
    InvalidChainLength(usize),
    #[error("Not Authenticated.")]
    NotAuthenticated,
    #[error("Failed to find ExtraUserdata")]
    ExtraUserdataNotFound,
    #[error("WrongSkinData:{0}")]
    WrongSkinData(String)
}
