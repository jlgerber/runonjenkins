use crate::constants::OS_VAR;
use shellfn::shell;

/// Retrieve the os from the shell
pub struct MachineOs;

impl MachineOs {
    /// Retreive the os from the environment if it exists.
    pub fn get_from_env() -> Option<String> {
        let my_os = _get_os(OS_VAR);
        my_os.ok()
    }
}

#[shell]
fn _get_os(os_var: &str) -> Result<String, failure::Error> {
    r#"
    printenv $OS_VAR
"#
}
