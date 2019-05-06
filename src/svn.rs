use crate::RemoteBuildError;
use crate::{prelude::*, ShellFnError};
use shellfn::shell;
use std::{env::current_dir, path::Path, path::PathBuf};
use log::debug;

/// Query for the url of the svn server. This struct requires that
/// the `svn` command be available on the shell to work. There are no
/// subversion clients in Rust so we use the excellent `shellfn` crate
/// to make quick work of this.
pub struct Svn;

impl Vcs for Svn {
    /// Test to see if the current working directory houses an svn repo.
    ///
    /// # Parameters
    ///
    /// None
    ///
    /// # Returns
    ///
    /// A bool indicating whether or not the current working directory
    /// is an svn repo or not.
    fn is_cwd_repo() -> bool {
        let cwd = current_dir().unwrap();
        Svn::is_repo(cwd)
    }

    /// Test to see if the provied directory houses an svn repo.
    ///
    /// # Parameters
    ///
    /// * `pathbuf` - The path to a directory which we intend on testing
    ///               to see if it houses an svn repository.
    ///
    /// # Returns
    ///
    /// Bool - indicates whether the repo exists
    fn is_repo<I: Into<PathBuf>>(pathbuf: I) -> bool {
        let mut pathbuf = pathbuf.into();
        pathbuf.push(".svn");
        pathbuf.exists()
    }

    /// Get the svn url from the manifest
    ///
    /// # Parameters
    ///
    /// * None
    ///
    /// # Returns
    ///
    /// A single element vector housing the Url to the svn server.
    fn get_server_urls(path: &Path) -> Result<Vec<url::Url>, failure::Error> {
        debug!("get_server_urls({:?})", path);
        // should probably unwrap this into an error
        let url_vec = _get_svn_url(path.to_str().unwrap_or("."))?;
        if url_vec.len() == 0 {
            Err(ShellFnError("unable to get svn url".to_string()).into())
        } else {
            Ok(vec![url::Url::parse(url_vec.as_str())?])
        }
    }
}

impl Svn {
    /// Get the svn url from the manifest
    ///
    /// # Parameters
    ///
    /// * `version`: &str representing an svn url version tag.
    ///
    /// # Returns
    ///
    /// A Url representing the svn server's url
    pub fn get_url(path: &Path, version: &str) -> Result<url::Url, RemoteBuildError> {
        debug!(
            "get_url calling get_server_urls with {:?}", path
        )
        let url = Svn::get_server_urls(path)?;
        let url = &url[0];
        let url = url::Url::parse(format!("{}/{}", url.as_str(), version).as_str())?;
        Ok(url)
    }
}

#[shell]
fn _get_svn_url(svn_base_path: &str) -> Result<String, failure::Error> {
    r#"
        cd $SVN_BASE_PATH && svn info --show-item url --no-newline | sed 's/trunk/tags/'
    "#
}
