use serde::de::{Deserializer, Deserialize, Error as SerdeError};
use self::super::super::util::BCP_47;
use self::super::super::Error;
use std::str::FromStr;
use std::ops::Deref;
use std::fmt;


/// A verified-valid BCP47 language tag.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LanguageTag(String);

impl FromStr for LanguageTag {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if BCP_47.is_match(s) {
            Ok(LanguageTag(s.to_string()))
        } else {
            Err(Error::Parse {
                tp: "BCP-47 language tag",
                wher: "language specifier".into(),
                more: format!("\"{}\" invalid", s).into(),
            })
        }
    }
}

impl<'de> Deserialize<'de> for LanguageTag {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        LanguageTag::from_str(<&'de str>::deserialize(deserializer)?).map_err(|e| {
            let buf = e.to_string();
            D::Error::custom(&buf[..buf.len() - 1]) // Drop dot
        })
    }
}

impl fmt::Display for LanguageTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for LanguageTag {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
