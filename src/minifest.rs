use shellfn::shell;
use crate::ShellFnError;

// for some odd reason, pk manifest --field does not work with --json flag
#[shell]
fn _get_minifest() -> Result<impl Iterator<Item=String>, failure::Error> { r#"
    pk manifest --field=name,version -b
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
}

/// retrieve the Minifest from disk
pub fn get_minifest() -> Result<Minifest, failure::Error> {
    let mut mmiter = _get_minifest()?;
    let name =  mmiter.next().ok_or_else(|| ShellFnError("unable to get name from manifest".to_string()))?;
    let version = mmiter.next().ok_or_else(|| ShellFnError("Unable to get version from manifest".to_string()))?;
    Ok(Minifest::new(name,version))
}
