use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize, Clone)]
pub enum Platform {
    #[serde(rename = "cent6_64")]
    Cent6,
    #[serde(rename = "cent7_64")]
    Cent7,
    Unknown(String),
}


impl Platform {
     
    /// Given a &str of potentially comma separated platform names,
    /// convert them to Platform instances, filtering out Platform::Unknowns
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
}

// Convert from a &str to a Platform using the Platform::from(...) syntax.
// This also comes into play with the Into<Platform> syntax. (as Into is defined
// generally for any types which implement From)
impl<'a> From<&'a str> for Platform {
    fn from(value: &'a str) -> Platform {
        match value.to_lowercase().as_str() {
            "cent6_64" | "cent6" => Platform::Cent6,
            "cent7_64" | "cent7" => Platform::Cent7,
            _ => Platform::Unknown(value.to_string()),
        }
    }
}

// Convert from a &Platform to a Platform using the Platform::from(...) syntax.
// This also comes into play with the Into<Platform> syntax. (as Into is defined
// generally for any types which implement From)
impl<'a> From<&'a Platform> for Platform {
    fn from(value: &'a Platform) -> Platform {
        value.clone()
    }
}

// convert from a Platform to a string using the Platform::from(...) syntax.
impl ToString for Platform {
    fn to_string(&self) -> String {
        match self {
            Platform::Cent6 => "cent6_64".to_string(),
            Platform::Cent7 => "cent7_64".to_string(),
            Platform::Unknown(val) => format!("unknown({})", val),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_platform_from_platform_reference() {
        use Platform::*;
        let tests = &[Cent6, Cent7];
        tests.iter().for_each(|test| {
            assert_eq!(Platform::from(test), test.clone());
        });
    }

    #[test]
    fn can_convert_from_str() {
        use Platform::*;
        let tests = &["cent6_64", "cent6", "Cent6", "cent7_64", "cent7", "Cent7"];

        let expected = &[Cent6, Cent6, Cent6, Cent7, Cent7, Cent7];

        tests.iter().enumerate().for_each(|(cnt, test)| {
            // have to dereference because `test` is a &&Platform
            // here ----------------->
            assert_eq!(Platform::from(*test), expected[cnt]);
        });
    }

    #[test]
    fn can_convert_to_string() {
        use Platform::*;
        let tests = &[Cent6, Cent7];
        let expected = &["cent6_64", "cent7_64"];
        tests.iter().enumerate().for_each(|(cnt, test)| {
            assert_eq!(test.to_string().as_str(), expected[cnt]);
        });
    }
}
