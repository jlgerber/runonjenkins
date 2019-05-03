

//use shellfn::shell;
//use crate::ShellFnError;
use git2::Repository;

/// Query the remote urls for a git repo, which we assume to be in the current working directory.
pub struct Git;

impl Git {
    // /// retrieve a list of remote urls from git
    // pub fn get_remote_urls() -> Result<Vec<url::Url>, failure::Error> {
    //     let urls = _get_git_remote_urls()?;
    //     // todo handle the unwrapping better. maybe use flatmap or...
    //     let urls = urls.map(|u| url::Url::parse(u.as_str()).unwrap()).collect::<Vec<url::Url>>();
    //     if urls.len() == 0 {
    //         Err(ShellFnError("Unable to get remote url for git".to_string()).into())
    //     } else {
    //         Ok(urls)
    //     }
    // }

    /// get remotes
    pub fn get_remotes(path: &str) -> Result<Vec<String>, failure::Error> {
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
}


// #[shell]
// fn _get_git_remote_urls() -> Result<impl Iterator<Item=String>, failure::Error> {
//     r#"
//         git remote -v | grep fetch | cut -f2 | cut -f1 -d " "
//     "#
// }