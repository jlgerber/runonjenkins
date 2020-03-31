use serde::{Deserialize, Serialize};
use std::string::ToString;
use crate::{Git,Svn, prelude::*};
use log::{debug,error};
use std::path::Path;

/// An enum whose variants represent common version control systems.
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Clone)]
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

impl<'a> From<&'a str> for VcsSystem {
    fn from(value: &'a str) -> Self {
        match value.to_lowercase().as_str() {
            "svn" => VcsSystem::Svn,
            "git" | "gitlab" => VcsSystem::Git,
            "mercurial" => VcsSystem::Mercurial,
            "perforce" => VcsSystem::Perforce,
            _ => VcsSystem::Unknown(value.to_string()),
        }
    }
}

// make sure that we can convert from a reference to self
impl<'a> From<&'a VcsSystem> for VcsSystem {
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

impl VcsSystem {
    // Given a reference to an Option<String> where the String is the vcs choice,
// and a path to the location where we are to look in the event that the selection is None,
// return a VcsSystem instance based either on supplied name (selection), or identification
// from the path. In the event that the user has supplied an invalid VcsSystem or path,
// this function will report an error to stderr and exit the process.
pub fn identify_vcs(selection: &Option<String>, path: &Path) -> VcsSystem {
    match selection {
        Some(val) => {
            debug!("vcs predefined");
            let vcs_val = VcsSystem::from(val.as_str());
            if let VcsSystem::Unknown(v) = vcs_val {
                error!("Unknown vcs system: {}", v);
                std::process::exit(1);
            }
            return vcs_val;
        }
        None => {
            if Git::is_repo(path) {
                debug!("git found");
                return VcsSystem::from("git");
            }
            if Svn::is_repo(path) {
                debug!("svn found");
                return VcsSystem::from("svn");
            }
        }
    }
    error!("Error: No VCS system idemtified");
    std::process::exit(1);
}

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_from_str_regardless_of_case() {
        let tests = &[
            "git",
            "GiT",
            "svn",
            "SVn",
            "mercurial",
            "MercUrial",
            "perforce",
            "Perforce",
            "foo",
        ];
        let expected = &[
            VcsSystem::Git,
            VcsSystem::Git,
            VcsSystem::Svn,
            VcsSystem::Svn,
            VcsSystem::Mercurial,
            VcsSystem::Mercurial,
            VcsSystem::Perforce,
            VcsSystem::Perforce,
            VcsSystem::Unknown("foo".to_string()),
        ];

        tests.iter().enumerate().for_each(|(cnt, test)| {
            assert_eq!(VcsSystem::from(*test), expected[cnt]);
        });
    }

    #[test]
    fn can_convert_to_string() {
        use VcsSystem::*;
        let tests = &[Git, Svn, Mercurial, Perforce, Unknown("Foo".to_string())];
        let expected = &["git", "svn", "mercurial", "perforce", "unknown(Foo)"];
        tests.iter().enumerate().for_each(|(cnt, test)| {
            assert_eq!(test.to_string().as_str(), expected[cnt]);
        });
    }

    #[test]
    fn can_build_from_vcssystem_reference() {
        use VcsSystem::*;
        let tests = &[Git, Svn, Mercurial, Perforce, Unknown("Foo".to_string())];

        tests.iter().for_each(|test| {
            assert_eq!(VcsSystem::from(test), test.clone());
        });
    }
}
