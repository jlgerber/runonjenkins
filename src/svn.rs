use shellfn::shell;
use crate::ShellFnError;

/// Query for the url of the svn server and that is about all.
pub struct Svn;

impl Svn {

    /// Get the svn url from the manifest
    ///
    /// # Parameters
    ///
    /// * `version`: &str representing an svn url version tag.
    ///
    /// # Returns
    ///
    /// A String representing the svn server's url
    pub fn get_url(version: &str) -> Result<String, failure::Error> {
        let url = _get_svn_url()?;
        if url.len() == 0 {
            Err(ShellFnError("unable to get svn url".to_string()).into())
        } else {
            Ok(format!("{}/{}", url, version))
        }
    }
}


#[shell]
fn _get_svn_url() -> Result<String, failure::Error> {
    r#"
        svn info --show-item url --no-newline | sed 's/trunk/tags/'
    "#
}
