
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
            "svn"  => VcsSystem::Svn,
            "git"  | "gitlab"  => VcsSystem::Git,
            "mercurial"        => VcsSystem::Mercurial,
            "perforce"         => VcsSystem::Perforce,
            _                  => VcsSystem::Unknown(value.to_string()),
        }
    }
}