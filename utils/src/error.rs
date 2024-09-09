//! Main Crate Error


#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    #[error("Invalid Identifier for deserialization")]
    IdentifierInvalid,
    #[error("Input for deserialization empty")]
    EmptyInput,
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("SerializeError")]
    SerializeError,

    // #[error[transparent]]
    // IO(#[from] std::io::Error),
}