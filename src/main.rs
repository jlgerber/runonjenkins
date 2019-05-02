

use yamly::*;

use shellfn::shell;
use std::error::Error;
use serde_json::{Value};
use serde::{Deserialize, Serialize};
use std::iter::Iterator;

#[derive(Debug, Deserialize, Serialize)]
struct Flavour {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Flavours {
    flavours: Vec<Flavour>
}

#[derive(Debug, Deserialize, Serialize)]
struct Manifests {
    manifests: Vec<Flavours>
}

#[shell]
fn _get_flavors() -> Result<impl Iterator<Item=String>, Box<Error>> { r#"
    pk manifest --flavours --name --json=1
"# }

pub fn get_flavors() -> Result<Vec<String>, Box<Error>> {
    let mut flavors = _get_flavors()?;
    let flavors = flavors.next().unwrap();
    let flavors: Manifests = serde_json::from_str(flavors.as_str())?;
    let result = flavors.manifests[0].flavours.iter().map(|flav| flav.name.to_string() ).collect::<Vec<String>>();
    Ok(result)
}

fn main() -> Result<(), Box<Error>>{
    let flavors = get_flavors()?;

    println!("{:?}", flavors);
    Ok(())
}
