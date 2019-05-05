use failure::Fail;
use std::fmt;

#[derive(Debug, Clone, Fail)]
pub struct ShellFnError(pub String);

impl fmt::Display for ShellFnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid result from shellfmt: {}", self.0)
    }
}

#[derive(Debug, Clone, Fail)]
pub struct RouteError(pub String);

impl fmt::Display for RouteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Problem with route: {}", self.0)
    }
}

#[derive(Fail, Debug)]
pub enum RemoteBuildError {
    #[fail(display = "Input was invalid UTF-8 at index {}", _0)]
    Utf8Error(usize),
    #[fail(display = "{}", _0)]
    Io(#[fail(cause)] std::io::Error),
    #[fail(display = "ConversionError: {}", _0)]
    ConversionError(String),
    #[fail(display = "None")]
    NoneError,
    #[fail(display = "ParseError: {}", _0)]
    ParseError(String),
    #[fail(display = "FailureError: {}", _0)]
    FailureError(String),
}

// make sure that we can convert from a reference to self
impl From<std::io::Error> for RemoteBuildError {
    fn from(value: std::io::Error) -> Self {
        RemoteBuildError::Io(value)
    }
}

// make sure that we can convert from a reference to self
impl From<failure::Error> for RemoteBuildError {
    fn from(value: failure::Error) -> Self {
        RemoteBuildError::FailureError(value.to_string())
    }
}

// make sure that we can convert from a reference to self
impl From<url::ParseError> for RemoteBuildError {
    fn from(value: url::ParseError) -> Self {
        RemoteBuildError::ParseError(value.to_string())
    }
}
// // make sure that we can convert from a reference to self
// impl From<std::option::NoneError> for RemoteBuildError {
//     fn from(value: std::option::NoneError) -> Self {
//        RemoteBuildError::NoneError
//     }
// }
