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
#[serde(untagged)]
pub enum BuildParamType {
    String(String),
    Platform(Platform),
    Url(String),
    Vcs(VcsSystem),
}

impl <'a> From<&'a str> for BuildParamType {
    fn from(value: &'a str) -> BuildParamType {
        BuildParamType::String(value.to_string())
    }
}


impl  From<String> for BuildParamType {
    fn from(value: String) -> BuildParamType {
        BuildParamType::String(value)
    }
}


impl  From<Url> for BuildParamType {
    fn from(value: Url) -> BuildParamType {
        BuildParamType::Url(value.to_string())
    }
}

impl From<Platform> for BuildParamType {
    fn from(value: Platform) -> BuildParamType {
        BuildParamType::Platform(value)
    }
}


impl From<VcsSystem> for BuildParamType {
    fn from(value: VcsSystem) -> BuildParamType {
        BuildParamType::Vcs(value)
    }
}

#[derive(Debug, PartialEq,PartialOrd,Eq,Ord,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildParameter {
    pub name: String,
    pub value: BuildParamType
}

impl BuildParameter {
    pub fn new<I,P>(name:I, value: P) -> BuildParameter
    where
        I: Into<String>,
        P: Into<BuildParamType> {
            Self {
                name: name.into(),
                value: value.into()
            }
        }
}

#[derive(Debug, PartialEq,PartialOrd,Eq,Ord,Serialize,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildParameters {
    pub parameter:Vec<BuildParameter>
}

impl BuildParameters {
    pub fn new() -> Self {
        Self{
            parameter: Vec::new()
        }
    }
}

#[derive(Debug, PartialEq,PartialOrd,Eq,Ord)]
pub struct BuildRequest {
    project: String,
    version: String,
    flavor: String,
    repo: Url,
    scm_type: VcsSystem,
    platform: Platform,
}

impl BuildRequest {
    pub fn new<'a,T,P>(
        project: T,
        version: T,
        flavor:  T,
        repo:   &'a str,
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
            repo: url,
            scm_type:scm_type.into(),
            platform: platform.into()
        })
    }

    /// generate a BuildParameters struct from a BuildRequest. The BuildParameters
    /// is json serializable and has the correct shape
    pub fn to_build_params(&self) -> BuildParameters {
        let mut params = BuildParameters::new();
        let project = BuildParameter::new("project", self.project.as_str());
        let version = BuildParameter::new("version",self.version.as_str());
        let flavor = BuildParameter::new("flavor", self.flavor.as_str());
        let repo = BuildParameter::new("repo", self.repo.clone()); //todo take 'a
        let scm_type = BuildParameter::new("ScmType", self.scm_type.clone());
        let platform = BuildParameter::new("platform", self.platform.clone());

        params.parameter.push(project);
        params.parameter.push(version);
        params.parameter.push(flavor);
        params.parameter.push(repo);
        params.parameter.push(scm_type);
        params.parameter.push(platform);

        params

    }
}


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

    #[test]
    fn can_serialize_to_json() {
        let req = BuildRequest::new(
            "houdini_submission",
            "5.4.0",
            "^",
            "http://dd-svn.d2.com/svn/software/packages/houdini_submission",
            "svn",
            "cent6"
        );
        let reqf = req.unwrap().to_build_params();
        let j = serde_json::to_string(&reqf).unwrap();
        assert_eq!(j, "{\"parameter\":[{\"name\":\"project\",\"value\":\"houdini_submission\"},{\"name\":\"version\",\"value\":\"5.4.0\"},{\"name\":\"flavor\",\"value\":\"^\"},{\"name\":\"repo\",\"value\":\"http://dd-svn.d2.com/svn/software/packages/houdini_submission\"},{\"name\":\"ScmType\",\"value\":\"Svn\"},{\"name\":\"platform\",\"value\":\"Cent6\"}]}");

    }
}

