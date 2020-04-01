use crate::{Minifest};
use std::path::Path;


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