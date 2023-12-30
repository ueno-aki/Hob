use serde::ser;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SerializeError {
    #[error("Unsupported: {0}")]
    Unsupported(String),
    #[error("DeserializeError:{0}")]
    Message(String),
}

impl ser::Error for SerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        SerializeError::Message(msg.to_string())
    }
}
