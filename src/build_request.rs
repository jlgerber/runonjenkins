//!
//! BuildRequest
//!
//! A BuildRequest models the data needed to trigger a build on jenkins. It includes the
//! project, the version, the flavor, teh repo, teh scm type
use crate::BuildParamType;
use crate::{Minifest, VcsSystem, Platform, RemoteBuildError};
use serde::{Serialize, Deserialize};
use serde;
use url::{ParseError,Url};
use crate::constants::*;
use std::str::FromStr;


#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An intermediate struct whose need is strictly dictated by the expected json
/// request's form. This struct diefines a name for a parameter and a value separately.
pub struct BuildParameter {
    pub name: String,
    pub value: BuildParamType,
}

impl BuildParameter {
    pub fn new<I, P>(name: I, value: P) -> BuildParameter
    where
        I: Into<String>,
        P: Into<BuildParamType>,
    {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

/// Intermedite structure that stores a list of `BuildParameter`s
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildParameters {
    pub parameter: Vec<BuildParameter>,
}

impl BuildParameters {
    /// Create a new, empty BuildParameters struct.
    pub fn new() -> Self {
        Self {
            parameter: Vec::with_capacity(PARAM_CNT),
        }
    }

    /// Push a BuildParameter instance into the BuildParameters struct
    pub fn push(&mut self, value: BuildParameter) {
        self.parameter.push(value);
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
/// The user facing request object. This is converted to the more cumbersome BuildParameters
/// object in order to serialize to json for the actual build request POST.
pub struct BuildRequest {
    /// name of the package
    pub project: String,
    /// version of the package, corresponding to an extant tag in the scm
    /// system that the package is stored in. For the build to be successful,
    /// the package must have been tagged under the `version`.
    pub version: String,
    /// package flavor. "^" is vanilla
    pub flavor: String,
    /// Url to the package's repository in version control
    pub repo: Url,
    /// The version control system that the package is stored in
    pub scm_type: VcsSystem,
    /// The os that the package is to be built for
    pub platform: Platform,
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
    pub fn new<'a, T, P>(
        project: T,
        version: T,
        flavor: T,
        repo: &'a str,
        scm_type: impl Into<VcsSystem>,
        platform: P,
    ) -> Result<Self, ParseError>
    where
        T: Into<String>,
        P: Into<Platform>,
    {
        let url = Url::from_str(repo)?;
        Ok(Self {
            project: project.into(),
            version: version.into(),
            flavor: flavor.into(),
            repo: url,
            scm_type: scm_type.into(),
            platform: platform.into(),
        })
    }

    /// Generate a BuildParameters struct from a BuildRequest. The BuildParameters
    /// is json serializable and has the correct shape
    pub fn to_build_params(&self) -> BuildParameters {
        let mut params = BuildParameters::new();
        let project = BuildParameter::new("project", self.project.as_str());
        let version = BuildParameter::new("version", self.version.as_str());
        let flavor = BuildParameter::new("flavor", self.flavor.as_str());
        let repo = BuildParameter::new("repo", self.repo.clone()); //todo take 'a
        let scm_type = BuildParameter::new("scmType", self.scm_type.clone());
        let platform = BuildParameter::new("platform", self.platform.clone());
        // From Rohith:
        // upstream_workspace is not being used. pass an empty string for now
        let upstream_workspace = BuildParameter::new("upstream_workspace", String::new());

        params.push(project);
        params.push(version);
        params.push(flavor);
        params.push(repo);
        params.push(scm_type);
        params.push(platform);
        params.push(upstream_workspace);
        
        params
    }

    // Construct a Vector of BuildRequest instances, one per flavor.
    // The BuildRequest provides a method that produces a struct
    // which is serializable into json in the form that Jenkins
    // is looking for
    pub fn build_requests(
        minifest: &Minifest,
        repo: &str,
        scm_type: &VcsSystem,
        platform: &Platform,
        flavors: &Vec<&str>,
    ) -> Result<Vec<BuildRequest>, RemoteBuildError> {
        let mut build_reqs = Vec::with_capacity(flavors.len());
        for flav in flavors {
            let build_request = BuildRequest::new(
                minifest.name.as_str(),
                minifest.version.as_str(),
                flav,
                repo,
                scm_type,
                platform,
            )?;
            build_reqs.push(build_request);
        }
        Ok(build_reqs)
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
            "cent6",
        );

        assert_eq!(
            req,
            Ok(BuildRequest {
                project: "houdini_submission".to_string(),
                version: "5.4.0".to_string(),
                flavor: "^".to_string(),
                scm_type: VcsSystem::Svn,
                repo: Url::from_str(
                    "http://dd-svn.d2.com/svn/software/packages/houdini_submission"
                )
                .unwrap(),
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
            "cent6",
        );
        let reqf = req.unwrap().to_build_params();
        let j = serde_json::to_string(&reqf).unwrap();
        assert_eq!(j, "{\"parameter\":[{\"name\":\"project\",\"value\":\"houdini_submission\"},{\"name\":\"version\",\"value\":\"5.4.0\"},{\"name\":\"flavor\",\"value\":\"^\"},{\"name\":\"repo\",\"value\":\"http://dd-svn.d2.com/svn/software/packages/houdini_submission\"},{\"name\":\"ScmType\",\"value\":\"Svn\"},{\"name\":\"platform\",\"value\":\"Cent6\"}]}");
    }
}

// pk manifest --falvours --jason=1
