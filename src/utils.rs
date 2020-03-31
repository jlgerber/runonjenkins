use crate::{Platform, Minifest, RemoteBuildError};
use log::{error};
use crate::flavor::get_flavors;
use std::path::Path;

// Convert a &str of comma separated flavor names into a
// vector of flavor name `&str`s
pub fn parse_flavors(flavor: &str) -> Vec<&str> {
    flavor.split(",").map(|x| x.trim()).collect::<Vec<&str>>()
}


// Given a &str of potentially comma separated platform names,
// convert them to Platform instances, filtering out Platform::Unknowns
pub fn parse_platforms(platforms: &str) -> Vec<Platform> {
    platforms
        .split(",")
        .map(|x| x.trim())
        .map(|x| Platform::from(x))
        // filter out any platforms that are unknown
        .filter(|x| {
            if let Platform::Unknown(_) = x {
                false
            } else {
                true
            }
        })
        .collect::<Vec<Platform>>()
}


// Given flavors and flavours options from the command line, reconcile the two and identify
// the requested flavors. This function will guard against specifying both flavors and flavours,
// exiting the process if neither is None.
// Furthermore, if both flavors and `flavours` are None, `resolve_flavors` will retrieve the
// full list of flavors from the manifest.
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
        get_flavors(path)?.join(".")
    } else if flavours.is_some() {
        flavours.unwrap()
    } else {
        flavors.unwrap()
    };
    Ok(flavors)
}


// get the minifest from the path, unless both the name and tag are passed in as Some. Then
// in that case, build the minifest out of them
pub fn get_minifest(
    project_path: &Path,
    name: &Option<String>,
    tag: &Option<String>,
) -> Result<Minifest, failure::Error> {
    if name.is_some() && tag.is_some() {
        let name = name.as_ref().unwrap();
        let tag = tag.as_ref().unwrap();
        Ok(Minifest::new(name.clone(), tag.clone()))
    } else {
        Minifest::from_disk(Some(&project_path))
    }
}