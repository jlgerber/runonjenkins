use std::collections::HashMap;
use crate::vcs_system::VcsSystem;
use strum_macros::{EnumString, Display, AsRefStr};
use std::str::FromStr;
use serde::{Deserialize};
//use serde_json::Result;
use crate::errors::RemoteBuildError;

/// A list of valid package types
#[derive(EnumString, Display, Debug, AsRefStr, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Deserialize)]
pub enum PackageType {
    #[strum(serialize="port", serialize="Port")]
    #[serde(rename="port")]
    Port,
    #[strum(serialize="source", serialize="Source")]
    #[serde(rename="source")]
    Source,
    #[strum(serialize="commercial", serialize="Commercial", serialize="comm")]
    #[serde(rename="commercial")]
    Commercial,
    Unknown
}

impl PackageType {
    pub fn is_valid(&self) -> bool {
        self != &Self::Unknown
    }
}
impl std::convert::From<&str> for PackageType {
    fn from(pkg_type: &str) -> Self {
        if let Ok(type_) = PackageType::from_str(pkg_type) {
            type_
        } else {
            Self::Unknown
        }

    }
}

#[derive(EnumString, Display, Debug, AsRefStr, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Deserialize)]
pub enum SourceStatus {
    #[strum(serialize="active", serialize="Active")]
    #[serde(rename="active")]
    Active,
    #[strum(serialize="retired", serialize="Retired")]
    #[serde(rename = "retired")]
    Retired,
    Unknown
}

impl SourceStatus {
    pub fn is_valid(&self) -> bool {
        self != &Self::Unknown
    }
}

impl std::convert::From<&str> for SourceStatus {
    fn from(stat: &str) -> Self {
        if let Ok(status) = SourceStatus::from_str(stat) {
            status
        } else {
            Self::Unknown
        }

    }
}

/// A Source represents a location for package source code within 
/// a vcs system, of which there may be multiple ones.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Deserialize)]
pub struct Source {
    #[serde(rename = "initSubmodules")]
    init_submodules: bool,
    link: String,
    status: SourceStatus,
    tags: String,
    uses: VcsSystem,
}

impl Source {
    /// Create a new Source from provided input data
    pub fn new<S: Into<String>, SS: Into<SourceStatus>, V: Into<VcsSystem> >( 
        init_submodules: bool,
        link: S,
        status: SS,
        tags: S,
        uses: V
    ) -> Self {
        Source {
            init_submodules, 
            link: link.into(),
            status: status.into(),
            tags: tags.into(),
            uses: uses.into()
        }
    }

    /// Determine whether the source is valid. An invalid source has an unknown status.
    pub fn is_valid(&self) -> bool {
        self.status.is_valid() && self.uses.is_valid()
    }
}

/// A record has a type and a list of sources.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Record {
    sources: Vec<Source>,
    #[serde(rename="type")]
    pkg_type: PackageType,
}

impl Record {
    /// New up an empty record.
    pub fn new<S: Into<PackageType> >(pkg_type: S) -> Self {
        Self {
            sources: Vec::new(),
            pkg_type: pkg_type.into()
        }
    }

    /// add a source to the list of sources
    pub fn add_source(&mut self, source: Source) -> &mut Self {
        self.sources.push(source);
        self
    }
    /// Retrieve an option wrapped reference to the source you request.
    /// Unlike index notation, this will not blow up if you pass an 
    /// out of bounds index, but you have to deal with unwrapping it.
    pub fn get(&self, idx: usize) -> Option<&Source> {
        if idx >= self.sources.len() {
            None
        } else {
            Some(&self.sources[idx])
        }
    }

    /// test whether any of the components are Unknown. 
    pub fn is_valid(&self) -> bool {
        if !self.pkg_type.is_valid() {return false;}
        for source in &self.sources {
            if !source.is_valid() {
                return false;
            }
        }
        true
    }
}

impl<'a> std::ops::Index<usize> for Record {
    type Output  = Source;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.sources[idx]
    }
}

/// GpiRecords type alias
pub type GpiRecordsType = HashMap<String, Record>;

/// root container for a set of one or more records.
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct GpiRecords {
    #[serde(flatten)]
    inner: GpiRecordsType
}

impl GpiRecords {
    /// Construct a Result wrapped serde_json
    // todo: convert to internal error type
    pub fn from_str(input: &str) -> Result<Self,RemoteBuildError> {//serde_json::Result<Self> {
        serde_json::from_str(input).map_err(|x| RemoteBuildError::SerdeJsonError(x))
    }

    /// retrieve the number of packages
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Determine whether a package is represented in the GpiRecords struct
    pub fn has(&self, package:&str) -> bool {
        self.inner.contains_key(package)
    }

    /// Retrieve a package Record by name.
    pub fn get(&self, package: &str) -> Option<&Record> {
        self.inner.get(package)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    // PackageType
    #[test]
    fn can_construct_package_type_from_string(){
        for p in &["port","Port", "source","Source", "commercial", "Commercial", "comm"] {
            let pt = PackageType::from_str(p);
            assert!(pt.is_ok());
        }
    }
    // SourceStatus
    #[test]
    fn can_construct_source_status_from_str() {
        for s in &["active", "retired"] {
            let stat: SourceStatus = (*s).into();
            assert!(stat.is_valid());
        }
    }

    #[test]
    fn can_catch_invalid_source_status() {
        let stat: SourceStatus = "flargyblargy".into();
        assert!(!stat.is_valid());
    }

    // Source
    #[test]
    fn can_construct_source_form_strs() {
        let src = Source::new(false, "http://pd-git.d2.com/fla", "active", "1.2.3", "git");
        assert!(src.is_valid());
    }
    #[test]
    fn can_catch_invalid_source() {
        let expect = Source::new(false, "foo", "active", "foo", "gitt");
        assert!(!expect.is_valid());
        let expect = Source::new(false, "foo", "blarney", "foo", "git");
        assert!(!expect.is_valid());
    }
    
    // Record
    #[test]
    fn can_catch_invalid_record_if_package_type_is_invalid() {
        let record = Record::new("bla");
        assert!(!record.is_valid());
        for value in &["port", "source", "commercial"] {
            let record = Record::new(*value);
            assert!(record.is_valid());
        }
    }

    #[test]
    fn can_catch_invalid_record_if_source_is_invalid() {
        let mut record = Record::new("port");
        record.add_source( Source::new(false, "foo", "active", "foo", "git")).add_source(Source::new(false, "foo", "blarney", "foo", "git"));
        assert!(!record.is_valid());
    }

    #[test]
    fn can_access_record_by_index(){
        let mut record = Record::new("port");
        record.add_source(Source::new(false, "bla", "active", "bla", "svn")).add_source(Source::new(false, "foo", "active", "foo", "git"));
        let expect = Source::new(false, "foo", "active", "foo", "git");
        assert_eq!(expect, record[1]);
        assert!(record[1].is_valid());
    }

    // GpiRecords
    #[test]
    fn can_create_gpi_records_from_str() {
        let data = 
r#"{
  "animtools": {
    "sources": [
      {
        "initSubmodules": false,
        "link": "ssh://git@dd-git.d2.com:2224/domains/animation/animtools.git",
        "status": "active",
        "subdirectory": "",
        "tags": "%",
        "uses": "git"
      }
    ],
    "type": "source"
  }
}"#;
        let results = GpiRecords::from_str(data);
        assert!(results.is_ok());
    }

    #[test]
    fn can_retrieve_record() {
        let data = 
r#"{
  "animtools": {
    "sources": [
      {
        "initSubmodules": false,
        "link": "ssh://git@dd-git.d2.com:2224/domains/animation/animtools.git",
        "status": "active",
        "subdirectory": "",
        "tags": "%",
        "uses": "git"
      }
    ],
    "type": "source"
  },
  "packalaka": {
    "sources": [
      {
        "initSubmodules": false,
        "link": "ssh://git@dd-git.d2.com:2224/domains/animation/packalaka.git",
        "status": "active",
        "subdirectory": "",
        "tags": "%",
        "uses": "git"
      }
    ],
    "type": "source"
  }
}"#;
        let results = GpiRecords::from_str(data).unwrap();
        let record = results.get("animtools");
        assert!(record.is_some());
    }
}
