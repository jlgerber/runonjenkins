
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug,PartialEq,PartialOrd,Eq,Ord,Serialize,Deserialize,Clone)]
pub enum VcsSystem {
    Svn,
    Git,
    Mercurial,
    Perforce,
    Unknown(String),
}

impl <'a> From<&'a str> for VcsSystem {

    fn from(value: &'a str) -> Self {
        match value.to_lowercase().as_str() {
            "svn"              => VcsSystem::Svn,
            "git"  | "gitlab"  => VcsSystem::Git,
            "mercurial"        => VcsSystem::Mercurial,
            "perforce"         => VcsSystem::Perforce,
            _                  => VcsSystem::Unknown(value.to_string()),
        }
    }
}


impl <'a> From<&'a VcsSystem> for VcsSystem {
    fn from(value: &'a VcsSystem) -> Self {
       value.clone()
    }
}


/*

impl  pkg_build_remote<String> for BuildParamType {
    fn from(value: String) -> BuildParamType {
        BuildParamType::String(value)
    }
}

*/