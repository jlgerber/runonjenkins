use pkg_build_remote::*;

use shellfn::shell;
use std::error::Error;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use std::iter::Iterator;

#[derive(Debug, Deserialize, Serialize)]
struct Flavour {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Flavours {
    flavours: Vec<Flavour>
}

#[derive(Debug, Deserialize, Serialize)]
struct Manifests {
    manifests: Vec<Flavours>
}

#[shell]
fn _get_flavors() -> Result<impl Iterator<Item=String>, Box<Error>> { r#"
    pk manifest --flavours --json=1
"# }

pub fn get_flavors() -> Result<Vec<String>, Box<Error>> {
    let mut flavors = _get_flavors()?;
    let flavors = flavors.next().unwrap();
    let flavors: Manifests = serde_json::from_str(flavors.as_str())?;
    let result = flavors.manifests[0].flavours.iter().map(|flav| flav.name.to_string() ).collect::<Vec<String>>();
    Ok(result)
}

// for some odd reason, pk manifest --field does not work with --json flag
#[shell]
fn _get_mini_mani() -> Result<impl Iterator<Item=String>, Box<Error>> { r#"
    pk manifest --field=name,version -b
"#
}

#[derive(Debug)]
/// The mini manifest - just the name and version because that is all we need.
pub struct MiniMani {
    pub name:    String,
    pub version: String,
}

impl MiniMani {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name, version
        }
    }
}

/// retrieve the MiniMani from disk
pub fn get_mini_mani() -> Result<MiniMani, Box<Error>> {
    let mut mmiter = _get_mini_mani()?;
    let name =  mmiter.next().ok_or_else(|| ShellFnError("unable to get name from manifest".to_string()))?;
    let version = mmiter.next().ok_or_else(|| ShellFnError("Unable to get version from manifest".to_string()))?;
    Ok(MiniMani::new(name,version))
}

//-----------------------------------
#[shell]
pub fn _get_svn_url() -> Result<String, Box<Error>> {
    r#"
        svn info --show-item url --no-newline | sed 's/trunk/tags/'
    "#
}

pub struct Svn;
impl Svn {

    /// Get the svn url from the manifest
    pub fn get_url(version: &str) -> Result<String, Box<Error>> {
        let url = _get_svn_url()?;
        if url.len() == 0 {
            Err(ShellFnError("unable to get svn url".to_string()).into())
        } else {
            Ok(format!("{}/{}", url, version))
        }
    }
}


#[shell]
pub fn _get_git_remote_urls() -> Result<impl Iterator<Item=String>, Box<Error>> {
    r#"
        git remote -v | grep fetch | cut -f2 | cut -f1 -d " "
    "#
}

pub struct Git;
impl Git {
  /// retrieve a list of remote urls from git
    pub fn get_remote_urls() -> Result<Vec<url::Url>, Box<Error>> {
        let urls = _get_git_remote_urls()?;
        // todo handle the unwrapping better. maybe use flatmap or...
        let urls = urls.map(|u| url::Url::parse(u.as_str()).unwrap()).collect::<Vec<url::Url>>();
        if urls.len() == 0 {
            Err(ShellFnError("Unable to get remote url for git".to_string()).into())
        } else {
            Ok(urls)
        }
    }
}


use std::error;
use std::fmt;

#[derive(Debug, Clone)]
struct ShellFnError(String);

impl fmt::Display for ShellFnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid result from shellfmt: {}", self.0)
    }
}

impl error::Error for ShellFnError {
    fn description(&self) -> &str {
        "invalid return from shellfn"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

//---------------------
//
//

fn main() -> Result<(), Box<Error>>{
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1] == "svn" {
            let remotes = Svn::get_url("1.2.3");
            println!("{:?}", remotes);
        } else if args[1] == "git" {
            let remotes = Git::get_remote_urls();
            println!("{:?}", remotes);
        } else {
            println!("choose svn or git");
        }
    } else {
        println!("usage: pkg_build_remote svn|git")
    }

    let flavors = get_flavors()?;
    println!("flavors: {:?}", flavors);
    let mini = get_mini_mani()?;
    println!("{:?}", mini);

    Ok(())
}

