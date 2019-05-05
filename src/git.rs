

//use shellfn::shell;
//use crate::ShellFnError;
use git2::Repository;
use std::{
    env::current_dir,
    path::PathBuf,
    path::Path,
};
use crate::traits;
use failure;

/// Query the remote urls for a git repo, which we assume to be in the current working directory.
pub struct Git;

impl traits::Vcs for Git {

    /// Test to see if the current directory houses a git repo.
    ///
    /// # Parameters
    ///
    /// None
    ///
    /// # Returns
    ///
    /// Bool indicating whether the current working directory
    /// houses a git repo or not.
    fn is_cwd_repo() -> bool{
        let cwd = current_dir().unwrap();
        Git::is_repo(cwd)
    }

    /// Test to see if the provied directory houses a git repo.
    ///
    /// # Parameters
    ///
    /// * `pathbuf` - The path to the directory which we wish to test.
    ///
    /// # Returns
    ///
    /// Bool indicating whether the supplied pathbuf houses a git repo or not
    fn is_repo<I: Into<PathBuf>>(pathbuf: I) -> bool {
        let mut pathbuf = pathbuf.into();
        pathbuf.push(".git");
        pathbuf.exists()
    }

    fn get_server_urls(path: &Path) -> Result<Vec<url::Url>, failure::Error> {

    /// get remote repositories for the local git repo in `path`.
    ///
    /// # Parameters
    ///
    /// * `path` - The path to the root of a git repository (ie it should have a .git folder in it)
    ///
    /// # Returns
    ///
        let repo =  Repository::init(path)?;
        Ok(repo.remotes()?
            .iter()
            .filter_map(|x| x) // remove Nones
            .map(|x| repo.find_remote(x).ok()) // get remotes, converting Result -> Option
            .filter_map(|x| x) // filter out None again
            .map(|x| x.url().unwrap_or("").to_string()) // get url, unwrapping and converting None -> ""
            .filter(|x| x != "") // filter out ""
            .map(|x| url::Url::parse(&x))
            .filter_map(Option::Some)
            .map(|x| x.unwrap())
            .collect()
        )

    }


}
impl Git {

    /// get remote repositories for the local git repo in `path`.
    ///
    /// # Parameters
    ///
    /// * `path` - The path to the root of a git repository (ie it should have a .git folder in it)
    ///
    /// # Returns
    ///
    pub fn get_remotes_strings(path: &str) -> Result<Vec<String>, failure::Error> {
        let repo =  Repository::init(path)?;
        Ok(repo.remotes()?
            .iter()
            .filter_map(|x| x) // remove Nones
            .map(|x| repo.find_remote(x).ok()) // get remotes, converting Result -> Option
            .filter_map(|x| x) // filter out None again
            .map(|x| x.url().unwrap_or("").to_string()) // get url, unwrapping and converting None -> ""
            .filter(|x| x != "") // filter out ""
            .collect()
        )
    }


    /// get remote repositories for the local git repo in `path`.
    ///
    /// # Parameters
    ///
    /// * `path` - The path to the root of a git repository (ie it should have a .git folder in it)
    ///
    /// # Returns
    ///
    pub fn get_remotes_urls(path: &str) -> Result<Vec<url::Url>, failure::Error> {
        let repo =  Repository::init(path)?;
        Ok(repo.remotes()?
            .iter()
            .filter_map(|x| x) // remove Nones
            .map(|x| repo.find_remote(x).ok()) // get remotes, converting Result -> Option
            .filter_map(|x| x) // filter out None again
            .map(|x| x.url().unwrap_or("").to_string()) // get url, unwrapping and converting None -> ""
            .filter(|x| x != "") // filter out ""
            .map(|x| url::Url::parse(&x))
            .filter_map(Option::Some)
            .map(|x| x.unwrap())
            .collect()
        )
    }
}
