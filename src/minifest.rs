use crate::{RemoteBuildError, ShellFnError};
use shellfn::shell;
use std::path::Path;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
/// The mini manifest - simply tracks the name and version, because that is what is relevant to us at this juncture.
pub struct Minifest {
    pub name: String,
    pub version: String,
}

impl Minifest {
    /// New up a Minifest, given a name and version that impl Into<String>
    pub fn new<I: Into<String>>(name: I, version: I) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
        }
    }

    /// Retrieve the Minifest from disk, assuming our CWD is in a
    /// project with a manifest.
    pub fn from_disk(path: Option<&Path>) -> Result<Minifest, failure::Error> {
        // convert path to &str
        let default_path = ".";
        let path = path.unwrap_or_else(|| Path::new(default_path));
        let path = path.as_os_str().to_str().ok_or_else(|| {
            RemoteBuildError::ConversionError("unable to convert path to str".to_string())
        })?;
        let mut mmiter = _get_minifest_from_grep(path)?;

        let mut attrs = Vec::with_capacity(2);
        attrs.push(
            mmiter
                .next()
                .ok_or_else(|| ShellFnError("Unable to get name from manifest. Perhaps the manifest was not found?".to_string()))?,
        );
        attrs.push(
            mmiter
                .next()
                .ok_or_else(|| ShellFnError("Unable to get version from manifest.".to_string()))?,
        );
        let mut name = String::new();
        let mut version = String::new();

        for attr in attrs {
            let mut results = attr.split(":");
            let key = results.next().unwrap().to_lowercase();
            if key == "name" {
                name = results.next().unwrap().to_string();
            } else if key == "version" {
                version = results.next().unwrap().to_string();
            }
        }

        Ok(Minifest::new(name, version))
    }
}

// for some odd reason, pk manifest --field does not work with --json flag
#[shell]
fn _get_minifest() -> Result<impl Iterator<Item = String>, failure::Error> {
    r#"
    pk manifest --field=name,version -b
"#
}

// pk manifest is quite a bit slowe than using unix commands to grab the version
// and name from the manifest
#[cfg(target_os = "macos")]
#[shell]
fn _get_minifest_from_grep(
    minifest_root_dir: &str,
) -> Result<impl Iterator<Item = String>, failure::Error> {
    r#"
    cd $MINIFEST_ROOT_DIR && gfind . -maxdepth 2 -regextype posix-egrep -regex '.*(manifest|pk)\.yaml' | xargs -I@ grep -iE '^version:|^name:' @ | sed s/\'//g | sed 's/ //g'
"#
}

#[cfg(target_os = "linux")]
#[shell]
fn _get_minifest_from_grep(
    minifest_root_dir: &str,
) -> Result<impl Iterator<Item = String>, failure::Error> {
    r#"
    cd $MINIFEST_ROOT_DIR && find . -maxdepth 2 -regextype posix-egrep -regex '.*(manifest|pk)\.yaml' | xargs -I@ grep -iE '^version:|^name:' @ | sed s/\'//g | sed 's/ //g'
"#
}

#[cfg(test)]
mod tests {
    use super::*;
    //use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;
    const MANIFEST: &'static str = r#"
---
name: 'Fred'
Version: '0.1.0'
Description: Other stuff
"#;

    #[test]
    fn can_fetch_from_disk() {
        let dir = std::env::temp_dir(); //.expect("could not create tempdir in test");
        let mut file_path = dir.clone();
        file_path.push("manifest.yaml");
        let mut file = File::create(&file_path).expect("could not create tempfile");
        writeln!(file, "{}", MANIFEST).expect("could not write MANIFEST to tempfile");

        let minifest = Minifest::from_disk(Some(&dir))
            .expect(format!("could not unwrap minifest in test: {:?}", dir).as_str());
        let expected = Minifest::new("Fred", "0.1.0");

        // remove file
        let _ = std::fs::remove_file(&file_path);

        assert_eq!(minifest, expected);
    }
}
