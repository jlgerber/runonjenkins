use url::{Url, ParseError};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::vcs_system::VcsSystem;
use std::default::Default;
use crate::constants::{
    BUILD_SERVER,
    BUILD_DOMAIN,
    BUILD_SERVER_PORT,
    BUILD_ROUTE,

};

#[derive(Debug,PartialEq,PartialOrd,Eq,Ord, Serialize, Deserialize)]
pub enum Platform {
    Cent6,
    Cent7,
    Unknown(String),
}

impl<'a> From<&'a str> for Platform {
    fn from(value: &'a str) -> Platform {
        match value.to_lowercase().as_str() {
            "cent6_64" | "cent6" => Platform::Cent6,
            "cent7_64" | "cent7" => Platform::Cent7,
            _ => Platform::Unknown(value.to_string())
        }
    }
}

#[derive(Debug, PartialEq,PartialOrd,Eq,Ord,Serialize,Deserialize)]
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

pub struct BuildServer {
    host: String,
    port: u32,
    domain: String,
}

impl BuildServer {
    pub fn new<I>(host:I, port: u32, domain: I)  -> Self
    where I: Into<String>
    {
        Self {
            host: host.into(),
            port,
            domain: domain.into(),
        }
    }
}

impl Default for BuildServer {
    fn default() -> Self {
        Self::new(BUILD_SERVER, BUILD_SERVER_PORT, BUILD_DOMAIN)
    }
}

pub fn make_request(req: &BuildRequest) -> Result<reqwest::Response,reqwest::Error> {
    let client = reqwest::Client::new();

    let res = client.post(req.repo.as_str())
    .json(req)
    .send();
    res
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
                 repo: Url::from_str("http://dd-svn.d2.com/svn/software/packages/houdini_submission").unwrap(),
                 platform: Platform::Cent6,
            })
        )
    }
}

