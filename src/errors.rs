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
}


// make sure that we can convert from a reference to self
impl From<std::io::Error> for RemoteBuildError {
    fn from(value: std::io::Error) -> Self {
       RemoteBuildError::Io(value)
    }
}


// // make sure that we can convert from a reference to self
// impl From<std::option::NoneError> for RemoteBuildError {
//     fn from(value: std::option::NoneError) -> Self {
//        RemoteBuildError::NoneError
//     }
// }