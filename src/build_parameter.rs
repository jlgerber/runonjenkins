use crate::BuildParamType;
//use crate::{VcsSystem, Platform, RemoteBuildError};
use serde::{Serialize, Deserialize};
use serde;
//use url::{ParseError,Url};
use crate::constants::*;
//use std::str::FromStr;
use log::debug;


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
        debug!("new BuildParameters instance");
        Self {
            parameter: Vec::with_capacity(PARAM_CNT),
        }
    }

    /// Push a BuildParameter instance into the BuildParameters struct
    pub fn push(&mut self, value: BuildParameter) {
        debug!("BuldParameters.push {:?}", &value);
        self.parameter.push(value);
    }
}
