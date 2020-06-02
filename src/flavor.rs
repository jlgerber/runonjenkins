use crate::RemoteBuildError;
use failure::AsFail;
use serde::{Deserialize, Serialize};
use shellfn::shell;
use std::{iter::Iterator, path::Path};
use log::error;



#[derive(Debug, Deserialize, Serialize)]
struct Flavour {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Flavours {
    flavours: Vec<Flavour>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Manifests {
    manifests: Vec<Flavours>,
}


impl Flavours {
    // Retrieve a list of flavors given an optional path to the
    // base git repo.
    fn get_flavors(path: Option<&Path>) -> Result<Vec<String>, failure::Error> {
        let default_path = ".";
        let path = path.unwrap_or_else(|| Path::new(default_path));
        let path = path.as_os_str().to_str().ok_or_else(|| {
            RemoteBuildError::ConversionError("unable to convert path to str".to_string())
        })?;

        let mut flavors = match _get_flavors(path) {
            Ok(val) => Ok(val),
            Err(e) => Err(RemoteBuildError::FlavorError(format!(
                "Failure shelling out to pk manifest: {}",
                e.as_fail()
            ))),
        }?;

        let flavors = flavors.next().ok_or(RemoteBuildError::EmptyError(
            "Unable to unwrap next flavor".into(),
        ))?;
        let flavors: Manifests = match serde_json::from_str(flavors.as_str()) {
            Ok(val) => Ok(val),
            Err(e) => Err(RemoteBuildError::FlavorError(format!(
                "Unable to retrieve flavors from manifest via pk manifest: {}",
                e.as_fail()
            ))),
        }?;
        
        assert!(flavors.manifests.len() > 0);

        let result = flavors.manifests[0]
            .flavours
            .iter()
            .map(|flav| flav.name.to_string())
            .collect::<Vec<String>>();
        Ok(result)
    }


    /// Convert a &str of comma separated flavor names into a
    /// vector of flavor name `&str`s
    pub fn parse_flavors(flavor: &str) -> Vec<&str> {
        flavor.split(",").map(|x| x.trim()).collect::<Vec<&str>>()
    }


    /// Given flavors and flavours options from the command line, reconcile the two and identify
    /// the requested flavors. This function will guard against specifying both flavors and flavours,
    /// exiting the process if neither is None.
    /// Furthermore, if both flavors and `flavours` are None, `resolve_flavors` will retrieve the
    /// full list of flavors from the manifest.
    /// If the manifest isnt provided, then we will default to vanilla
    pub fn resolve_flavors(
        flavors: Option<String>,
        flavours: Option<String>,
        path: Option<&std::path::Path>,
    ) -> Result<String, RemoteBuildError> {
        if flavours.is_some() && flavors.is_some() {
            error!("Using --falvours and --flavors? You cheeky monkey. Pick one or the other");
            std::process::exit(1);
        }

        let flavors = if flavours.is_none() && flavours.is_none() {
            // think that this should be comma separated not dot...changing
            Self::get_flavors(path).unwrap_or(vec!["^".into()]).join(",")
        } else if flavours.is_some() {
            flavours.unwrap()
        } else {
            flavors.unwrap()
        };
        Ok(flavors)
    }
}



#[shell]
fn _get_flavors(flavor_path: &str) -> Result<impl Iterator<Item = String>, failure::Error> {
    r#"
    cd $FLAVOR_PATH && pk manifest --flavours --json=1
"#
}
