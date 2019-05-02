
/*
curl -X POST
http://pd-docker-nd-08.d2.com:5000/view/DDPipeline/job/Plans/job/BuildTagPipeline/build
--data-urlencode json='{"parameter": [{"name":"project", "value":"houdini_submission"}]}'
 */

use yamly::*;

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

fn parse() {
 let result: ManifestMap = serde_yaml::from_reader(VAL.as_bytes()).unwrap();
 if let YamlValue::String(ref name) = result["name"] {
    println!("name {}", name);
 }
    let res: Manifest = serde_yaml::from_reader(VAL.as_bytes()).unwrap();
    println!("{:?}", res);
 // println!("{:?}", result["name"]);

 //println!("{:?}", result);
}
fn main() {
    parse();
    println!("Hello, world!");
}
