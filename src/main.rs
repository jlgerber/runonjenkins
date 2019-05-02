

use yamly::*;
use std::io::Read;
use serde_yaml ;
use serde::{Deserialize, };
use std::collections::BTreeMap ;
use serde_yaml::Value as YamlValue;
pub mod build_request;
type ManifestMap = BTreeMap<String, YamlValue>;

#[derive(Deserialize,Debug)]
struct Manifest {
    name: String,
}

const VAL:  &'static str = r#"
---
name: foobar
version: 1
description: The coolest app ever
requires:
    foo: '1.0.0+<3'
    bar: '2.0.0+'
"#;

pub enum ManifestType {
    Str(&'static str),
    File(String),
}

fn package_from_manifest(manifest: &str) -> Option<String> {
    match serde_yaml::from_reader(manifest.as_bytes()){
        Ok(val) => Some(val),
        Err(_) => None
    }
}


fn main() {
    let some_package = package_from_manifest(VAL);
    println!("{:?}", some_package);
}
