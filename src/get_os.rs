use shellfn::shell;
use crate::{ShellFnError, constants::OS_VAR;

/// Retrieve the os from the shell
pub struct MachineOs;

impl MachineOs {
    /// Retreive the os from the environment
    pub fn get_from_env() -> String {
        let my_os = _get_os(OS_VAR);
    }
}

#[shell]
fn _get_os(os_var: &str) -> Result<String, failure::Error> { r#"
    printenv $OS_VAR
"#
}