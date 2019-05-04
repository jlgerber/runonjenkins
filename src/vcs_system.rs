
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Debug,PartialEq,PartialOrd,Eq,Ord,Serialize,Deserialize,Clone)]
pub enum VcsSystem {
    #[serde(rename = "svn")]
    Svn,
    #[serde(rename = "git")]
    Git,
    #[serde(rename = "mercurial")]
    Mercurial,
    #[serde(rename = "perforce")]
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

impl ToString for VcsSystem {
    fn to_string(&self) -> String {
        match self {
            VcsSystem::Svn => "svn".to_string(),
            VcsSystem::Git => "git".to_string(),
            VcsSystem::Mercurial => "mercurial".to_string(),
            VcsSystem::Perforce => "perforce".to_string(),
            VcsSystem::Unknown(value) => format!("unknown({})", value),
        }
    }
}

/*

impl  pkg_build_remote<String> for BuildParamType {
    fn from(value: String) -> BuildParamType {
        BuildParamType::String(value)
    }
}

*/