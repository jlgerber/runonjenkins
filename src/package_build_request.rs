//!
//! PackageBuildRequest
//!
//! A PackageBuildRequest models the data needed to trigger a build on jenkins. It includes the
//! project, the version, the flavor, the repo, teh scm type
//use crate::BuildParamType;
use crate::{
    //RemoteBuildError, 
    BuildParameters, 
    BuildParameter,
    errors::RemoteBuildError
};

use log::debug;


#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
/// The user facing request object. This is converted to the more cumbersome BuildParameters
/// object in order to serialize to json for the actual build request POST.
pub struct PackageBuildRequest {
    /// name of the package
    pub project: String,
    /// tag we wish to build
    pub tag: String,
}

impl PackageBuildRequest {
    /// Generate a new PackageBuildRequest.
    pub fn new<'a, T>(
        project: T,
        tag: T,
    ) -> Self
    where
        T: Into<String> + std::fmt::Debug,
    {
        debug!("PackageBuildRequest::new({:?}, {:?})", project, tag);
        Self {
            project: project.into(),
            tag: tag.into(),
        }
    }

    /// Generate a BuildParameters struct from a PackageBuildRequest. The BuildParameters
    /// is json serializable and has the correct shape
    pub fn to_build_params(&self) -> BuildParameters {
        let mut params = BuildParameters::new();
        let project = BuildParameter::new("project", self.project.as_str());
        let tag = BuildParameter::new("tag", self.tag.as_str());

        // From Rohith:
        // upstream_workspace is not being used. pass an empty string for now
        //let upstream_workspace = BuildParameter::new("upstream_workspace", String::new());

        params.push(project);
        params.push(tag);    
        params
    }

    // // Construct  PackageBuildRequest
    // // The PackageBuildRequest provides a method that produces a struct
    // // which is serializable into json in the form that Jenkins
    // // is looking for
    pub fn build_request(
        name: &str,
        tag: &str,
       
    ) -> Result<PackageBuildRequest, RemoteBuildError> {

        Ok(PackageBuildRequest::new(name, tag))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_req() {
        let req = PackageBuildRequest::new(
            "houdini_submission",
            "5.4.0",
           
        );

        assert_eq!(
            req,
            Ok(PackageBuildRequest {
                project: "houdini_submission".to_string(),
                tag: "5.4.0".to_string(),
            })
        )
    }

    #[test]
    fn can_serialize_to_json() {
        let req = PackageBuildRequest::new(
            "houdini_submission",
            "5.4.0",
        );
        let reqf = req.unwrap().to_build_params();
        let j = serde_json::to_string(&reqf).unwrap();
        assert_eq!(j, "{\"parameter\":[{\"name\":\"project\",\"value\":\"houdini_submission\"},{\"name\":\"tag\",\"value\":\"5.4.0\"}");
    }
}

// pk manifest --falvours --jason=1
