use std::path::{PathBuf, Path};
use failure;

pub trait Vcs {
    /// is the current working directory a repository?
    fn is_cwd_repo() -> bool;
    /// is the supplied path buffer a repository?
    fn is_repo<I: Into<PathBuf>>(pathbuf: I) -> bool;
    /// retrieve server url(s)
    fn get_server_urls(path: &Path) -> Result<Vec<url::Url>, failure::Error>;
}