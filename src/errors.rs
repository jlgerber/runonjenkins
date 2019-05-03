use failure::Fail;
use std::fmt;

#[derive(Debug, Clone, Fail)]
pub struct ShellFnError(pub String);

impl fmt::Display for ShellFnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid result from shellfmt: {}", self.0)
    }
}
