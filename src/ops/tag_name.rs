use serde::de::{Deserializer, Deserialize, Error as SerdeError};
use self::super::super::Error;
use std::str::FromStr;
use std::ops::Deref;
use std::fmt;


/// A verified-valid post tag.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TagName(String);

impl FromStr for TagName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if !s.contains(|c: char| c.is_whitespace() || c.is_control()) {
            Ok(TagName(s.to_string()))
        } else {
            Err(Error::Parse {
                tp: "WS- and controlless string",
                wher: "post tag name".into(),
                more: Some(format!("\"{}\" invalid", s).into()),
            })
        }
    }
}

impl<'de> Deserialize<'de> for TagName {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        TagName::from_str(<&'de str>::deserialize(deserializer)?).map_err(|e| {
            let mut buf = vec![];
            e.print_error(&mut buf);
            D::Error::custom(String::from_utf8_lossy(&buf[..buf.len() - 2])) // Drop dot and newline
        })
    }
}

impl fmt::Display for TagName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for TagName {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}
