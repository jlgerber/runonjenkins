//!
//! BuildRequest
//!
//! A BuildRequest models the data needed to trigger a build on jenkins. It includes the
//! project, the version, the flavor, teh repo, teh scm type
use url::{Url, ParseError};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::VcsSystem;
use crate::Platform;

#[derive(Debug, PartialEq,PartialOrd,Eq,Ord,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildRequest {
    project: String,
    version: String,
    flavor: String,
    repo: String,
    scm_type: VcsSystem,
    platform: Platform,
}

impl BuildRequest {
    pub fn new<'a,T,P>(
        project: T,
        version: T,
        flavor:  T,
        repo:    &'a str,
        scm_type: impl Into<VcsSystem>,
        platform: P
    )-> Result<Self, ParseError>
    where
        T : Into<String> ,
        P : Into<Platform>,
     {
        let url = Url::from_str(repo)?;
        Ok(Self {
            project: project.into(),
            version: version.into(),
            flavor: flavor.into(),
            repo: url.to_string(),
            scm_type:scm_type.into(),
            platform: platform.into()
        })
    }
}


// project   houdini_submission
// version   5.4.0
// flavor    ^
// scmType   svn
// repo      http://dd-svn.d2.com/svn/software/packages/houdini_submission
// platform  Cent7_64



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_req() {
        let req = BuildRequest::new(
            "houdini_submission",
            "5.4.0",
            "^",
            "http://dd-svn.d2.com/svn/software/packages/houdini_submission",
            "svn",
            "cent6"
        );

        assert_eq!(
            req,
            Ok(BuildRequest {
                project: "houdini_submission".to_string(),
                version: "5.4.0".to_string(),
                flavor: "^".to_string(),
                scm_type: VcsSystem::Svn,
                 repo: "http://dd-svn.d2.com/svn/software/packages/houdini_submission".to_string(),
                 platform: Platform::Cent6,
            })
        )
    }
}

