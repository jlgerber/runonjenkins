use crate::Platform;
use crate::VcsSystem;
use serde::{Deserialize, Serialize};
use url::Url;

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
impl<'a> From<&'a str> for BuildParamType {
    fn from(value: &'a str) -> BuildParamType {
        BuildParamType::String(value.to_string())
    }
}

impl From<String> for BuildParamType {
    fn from(value: String) -> BuildParamType {
        BuildParamType::String(value)
    }
}

impl From<Url> for BuildParamType {
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
