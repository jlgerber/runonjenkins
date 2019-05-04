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
use crate::constants::PARAM_CNT;
use std::string::ToString;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(untagged)]
/// An enumeration of possible build parameter types. These include string, platform, url, and vcs system.
pub enum BuildParamType {
    String(String),
    Platform(Platform),
    Url(String),
    Vcs(VcsSystem),
}

// From conversions for BuildParamType
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

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An intermediate struct whose need is strictly dictated by the expected json
/// request's form. This struct diefines a name for a parameter and a value separately.
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

/// Intermedite structure that stores a list of `BuildParameter`s
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildParameters {
    pub parameter:Vec<BuildParameter>
}

impl BuildParameters {
    /// Create a new, empty BuildParameters struct.
    pub fn new() -> Self {
        Self{
            parameter: Vec::with_capacity(PARAM_CNT)
        }
    }

    /// Push a BuildParameter instance into the BuildParameters struct
    pub fn push(&mut self, value: BuildParameter) {
        self.parameter.push(value);
    }
}

#[derive(Debug, PartialEq,PartialOrd,Eq,Ord)]
/// The user facing request object. This is converted to the more cumbersome BuildParameters
/// object in order to serialize to json for the actual build request POST.
pub struct BuildRequest {
    /// name of the package
    project: String,
    /// version of the package, corresponding to an extant tag in the scm
    /// system that the package is stored in. For the build to be successful,
    /// the package must have been tagged under the `version`.
    version: String,
    /// package flavor. "^" is vanilla
    flavor: String,
    /// Url to the package's repository in version control
    repo: Url,
    /// The version control system that the package is stored in
    scm_type: VcsSystem,
    /// The os that the package is to be built for
    platform: Platform,
}

impl BuildRequest {

    /// Generate a new BuildRequest.
    ///
    /// # Parameters
    ///
    /// * `project` - Name of the package, as a type which can be converted into a String.
    /// * `version` - Version of the package, which must also be an extant tag in the vcs.
    /// * `flavor`  - Specific flavor we are requesting be built.
    /// * `repo`    - Url to the project.
    /// * `scm_type` - The type of the Version Control System that the tagged project is checked in to.
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

    /// Generate a BuildParameters struct from a BuildRequest. The BuildParameters
    /// is json serializable and has the correct shape
    pub fn to_build_params(&self) -> BuildParameters {
        let mut params = BuildParameters::new();
        let project = BuildParameter::new("project", self.project.as_str());
        let version = BuildParameter::new("version",self.version.as_str());
        let flavor = BuildParameter::new("flavor", self.flavor.as_str());
        let repo = BuildParameter::new("repo", self.repo.clone()); //todo take 'a
        let scm_type = BuildParameter::new("scmType", self.scm_type.clone());
        let platform = BuildParameter::new("platform", self.platform.clone());

        params.push(project);
        params.push(version);
        params.push(flavor);
        params.push(repo);
        params.push(scm_type);
        params.push(platform);

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

// pk manifest --falvours --jason=1