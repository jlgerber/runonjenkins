use crate::RemoteBuildError;
use serde::{Deserialize, Serialize};
use shellfn::shell;
use std::{iter::Iterator, path::Path};
use failure::AsFail;


/// Retrieve a list of flavors given an optional path to the
/// base git repo.
pub fn get_flavors(path: Option<&Path>) -> Result<Vec<String>, failure::Error> {
    let default_path = ".";
    let path = path.unwrap_or_else(|| Path::new(default_path));
    let path = path.as_os_str().to_str().ok_or_else(|| {
        RemoteBuildError::ConversionError("unable to convert path to str".to_string())
    })?;

    let mut flavors = match _get_flavors(path) {
        Ok(val) => Ok(val),
        Err(e) =>
            Err(RemoteBuildError::FlavorError(
                format!("Failure shelling out to pk manifest: {}", e.as_fail()))
            )
    }?;

    let flavors = flavors.next().unwrap();
    let flavors: Manifests = match serde_json::from_str(flavors.as_str()){
        Ok(val) => Ok(val),
        Err(e) =>
            Err(RemoteBuildError::FlavorError(
                format!("Unable to retrieve flavors from manifest via pk manifest: {}", e.as_fail()))
            )
    }?;

    let result = flavors.manifests[0]
        .flavours
        .iter()
        .map(|flav| flav.name.to_string())
        .collect::<Vec<String>>();
    Ok(result)
}


#[derive(Debug, Deserialize, Serialize)]
struct Flavour {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Flavours {
    flavours: Vec<Flavour>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Manifests {
    manifests: Vec<Flavours>,
}

#[shell]
fn _get_flavors(flavor_path: &str) -> Result<impl Iterator<Item = String>, failure::Error> {
    r#"
    cd $FLAVOR_PATH && pk manifest --flavours --json=1
"#
}