use serde::{Deserialize};
use crate::errors::RemoteBuildError;
use shellfn::shell;
use log::debug;
use crate::gpi::SourceStatus;
use url::{ParseError,Url};
use crate::vcs_system::VcsSystem;
use std::str::FromStr;

/// packalaka tags --json <name> <tag> returns
/// a list of these
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Deserialize)]
pub struct PackageTag {
    pub link:String,
    pub name: String,
    pub uses: VcsSystem,
    pub status: SourceStatus,
    versions: Vec<String>
}

impl PackageTag {
    /// Construct a PcakageTag from a &str. 
    pub fn from_str(input: &str) -> Result<Self,RemoteBuildError> {//serde_json::Result<Self> {
        serde_json::from_str(input).map_err(|x| RemoteBuildError::SerdeJsonError(x))
    }

    /// Retrieve a list of flavors for a given Packalaka version
    pub fn flavors(&self) -> Vec<&str> {
        let splitter = format!("{}_", &self.name);
        self.versions.iter().map(move |x| {
            let r=x.split(&splitter).last().unwrap_or("^");
            if r == self.name {
                "^"
            } else {
                r
            } 

        }).collect::<Vec<&str>>()
    }

    /// Retrieve the url
    pub fn link(&self) -> Result<Url, RemoteBuildError> {
        Ok(Url::from_str(&self.link)?)
    }
}

/// root container for a set of one or more records.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PackageTagList {
    inner: Vec<PackageTag>
}

impl PackageTagList {
    /// Construct a Result wrapped serde_json
    // todo: convert to internal error type
    pub fn from_str(input: &str) -> Result<Self,RemoteBuildError> {//serde_json::Result<Self> {
        let lst: Vec<PackageTag> = serde_json::from_str(input).map_err(|x| RemoteBuildError::SerdeJsonError(x))?;
        Ok(PackageTagList{inner:lst})
    }
    /// retrieve info from packalaka service
    pub fn from_service(package: &str, tag: &str) -> Result<Self, RemoteBuildError> {
        debug!("shelling out to packalaka tags --json {} {}", package, tag);
        let package_str = _get_packalaka(package, tag).map_err(|e| RemoteBuildError::ShellFnError(format!("{:?}",e)))?;
        Self::from_str(&package_str)
    }

    /// retrieve the number of PackageTags in the verison list
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Retrieve a PackageTag by index.
    pub fn get(&self, idx: usize) -> Option<&PackageTag> {
        if self.inner.len() > idx {

            Some(&self.inner[idx])
        } else {
            None
        }
    }
}


#[shell]
fn _get_packalaka(package_name: &str, package_tag: &str) -> Result<String, shellfn::Error<std::convert::Infallible>> {
    r#"
        packalaka tags --json --skip-pre $PACKAGE_NAME $PACKAGE_TAG
    "#
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn can_create_version_from_str() {
        let data =
r#"
{
    "link": "ssh://git@dd-git.d2.com:2224/domains/lighting/deferredpipeline.git#tag=3.5.0.alpha2",
    "name": "3.5.0.alpha2",
    "status": "active",
    "versions": [
      "3.5.0.alpha2_vray3.6.27936_for_maya2018",
      "3.5.0.alpha2_vray4.0.29259_for_maya2018",
      "3.5.0.alpha2_vray4.0.29935_for_maya2018"
    ]
  }

"#;
        let results = PackageTag::from_str(data);
        assert!(results.is_ok());
    }
    #[test]
    fn can_create_versionlist_from_str() {
        let data = 
r#"[
  {
    "link": "ssh://git@dd-git.d2.com:2224/domains/lighting/deferredpipeline.git#tag=3.5.0",
    "name": "3.5.0",
    "status": "active",
    "versions": [
      "3.5.0_vray3.6.27936_for_maya2018",
      "3.5.0_vray4.0.29259_for_maya2018"
    ]
  },
  {
    "link": "ssh://git@dd-git.d2.com:2224/domains/lighting/deferredpipeline.git#tag=3.5.0.alpha1",
    "name": "3.5.0.alpha1",
    "status": "active",
    "versions": [
      "3.5.0.alpha1_vray3.6.27936_for_maya2018",
      "3.5.0.alpha1_vray4.0.29259_for_maya2018",
      "3.5.0.alpha1_vray4.0.29935_for_maya2018"
    ]
  },
  {
    "link": "ssh://git@dd-git.d2.com:2224/domains/lighting/deferredpipeline.git#tag=3.5.0.alpha2",
    "name": "3.5.0.alpha2",
    "status": "active",
    "versions": [
      "3.5.0.alpha2_vray3.6.27936_for_maya2018",
      "3.5.0.alpha2_vray4.0.29259_for_maya2018",
      "3.5.0.alpha2_vray4.0.29935_for_maya2018"
    ]
  }
]"#;
        let results = PackageTagList::from_str(data);
        assert!(results.is_ok());
    }
}
