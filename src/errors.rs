use thiserror::Error;

#[derive(Error, Debug)]
pub enum RemoteBuildError {
    #[error("Input was invalid UTF-8 at index {0}")]
    Utf8Error(usize),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("ConversionError: {0}")]
    ConversionError(String),
    #[error("serde_json build error: {0:?}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("NoneError")]
    NoneError,
    #[error("ShellFnError {0}")]
    ShellFnError(String ),
    #[error("EmptyError: {0}")]
    EmptyError(String),
    #[error("ParseError: {0}")]
    ParseError(#[from] url::ParseError),
    #[error("FlavorError: {0}")]
    FlavorError(String),
    #[error("FailureError: {0}")]
    FailureError(String),
    #[error("Gpi Record Failure {0}")]
    GpiRecordFailure(String),
}

// make sure that we can convert from a reference to self
impl From<failure::Error> for RemoteBuildError {
    fn from(value: failure::Error) -> Self {
        RemoteBuildError::FailureError(value.to_string())
    }
}