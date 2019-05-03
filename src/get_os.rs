use shellfn::shell;
use crate::ShellFnError;


pub struct MachineOs;

impl MachineOs {
    pub fn get_from_env() -> String {
        let my_os = _get_os();
    }
}

#[shell]
fn _get_os() -> Result<String, failure::Error> { r#"
    printenv DD_OS
"#
}