use std::collections::HashMap;
use crate::vcs_system::VcsSystem;
use strum_macros::{EnumString, Display, AsRefStr};
use std::str::FromStr;
use serde::{Deserialize};
use serde_json::Result;

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
    pub fn new<S: AsRef<str> >(pkg_type: S) -> Self {
        Self {
            sources: Vec::new(),
            pkg_type: PackageType::from_str(pkg_type.as_ref()).unwrap()
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
}

impl<'a> std::ops::Index<usize> for Record {
    type Output  =Source;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.sources[idx]
    }
}

/// GpiRecords type alias
pub type GpiRecords = HashMap<String, Record>;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_access_record_by_index(){
        let mut record = Record::new("port");
        record.add_source(Source::new(false, "bla", "bla", "bla", "svn")).add_source(Source::new(false, "foo", "foo", "foo", "git"));
        let expect = Source::new(false, "foo", "foo", "foo", "git");
        assert_eq!(expect, record[1]);
    }


    #[test]
    fn can_construct_package_type_from_string(){
        for p in &["port","Port", "source","Source", "commercial", "Commercial", "comm"] {
            let pt = PackageType::from_str(p);
            assert!(pt.is_ok());
        }
    }
    
}
