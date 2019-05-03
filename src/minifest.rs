use shellfn::shell;
use crate::ShellFnError;

// for some odd reason, pk manifest --field does not work with --json flag
#[shell]
fn _get_minifest() -> Result<impl Iterator<Item=String>, failure::Error> { r#"
    pk manifest --field=name,version -b
"#
}

#[cfg(target_os = "macos")]
#[shell]
fn _get_minifest_from_grep() -> Result<impl Iterator<Item=String>, failure::Error> { r#"
    gfind . -regextype posix-egrep -regex '.*(manifest|pk)\.yaml' | xargs -I@ grep -iE '^version:|^name:' @ | sed s/\'//g | sed 's/ //g'
"#
}

#[cfg(target_os = "linux")]
#[shell]
fn _get_minifest_from_grep() -> Result<impl Iterator<Item=String>, failure::Error> { r#"
    find . -regextype posix-egrep -regex '.*(manifest|pk)\.yaml' | xargs -I@ grep -iE '^version:|^name:' @ | sed s/\'//g | sed 's/ //g'
"#
}

#[derive(Debug)]
/// The mini manifest - just the name and version because that is all we need.
pub struct Minifest {
    pub name:    String,
    pub version: String,
}

impl Minifest {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name, version
        }
    }

    /// retrieve the Minifest from disk
    pub fn from_disk() -> Result<Minifest, failure::Error> {
        let mut mmiter = _get_minifest_from_grep()?;
        let mut attrs = Vec::with_capacity(2);
        attrs.push( mmiter.next().ok_or_else(|| ShellFnError("unable to get name from manifest".to_string()))? );
        attrs.push(mmiter.next().ok_or_else(|| ShellFnError("Unable to get version from manifest".to_string()))?);
        let mut name = String::new();
        let mut version = String::new();

        for attr in attrs {
            let mut results = attr.split(":");
            let key = results.next().unwrap().to_lowercase();
            if key == "name" {
                name = results.next().unwrap().to_string();
            }else if key == "version" {
                version = results.next().unwrap().to_string();
            }
        }

        Ok(Minifest::new(name,version))
    }
    // /// retrieve the Minifest from disk
    // pub fn from_disk() -> Result<Minifest, failure::Error> {
    //     let mut mmiter = _get_minifest()?;
    //     let name =  mmiter.next().ok_or_else(|| ShellFnError("unable to get name from manifest".to_string()))?;
    //     let version = mmiter.next().ok_or_else(|| ShellFnError("Unable to get version from manifest".to_string()))?;
    //     Ok(Minifest::new(name,version))
    // }
}


