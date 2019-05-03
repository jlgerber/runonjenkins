use serde::{Serialize, Deserialize};

#[derive(Debug,PartialEq,PartialOrd,Eq,Ord, Serialize, Deserialize,Clone)]
pub enum Platform {
    Cent6,
    Cent7,
    Unknown(String),
}

impl<'a> From<&'a str> for Platform {
    fn from(value: &'a str) -> Platform {
        match value.to_lowercase().as_str() {
            "cent6_64" | "cent6" => Platform::Cent6,
            "cent7_64" | "cent7" => Platform::Cent7,
            _ => Platform::Unknown(value.to_string())
        }
    }
}


impl<'a> From<&'a Platform> for Platform {
    fn from(value: &'a Platform) -> Platform {
       value.clone()
    }
}
