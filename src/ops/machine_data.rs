use serde::de::{Deserializer, Deserialize, Error as SerdeError};
use std::collections::BTreeMap;
use self::super::super::Error;
use std::str::FromStr;
use unicase::UniCase;
use std::ops::Deref;
use std::fmt;


lazy_static! {
    static ref NAME_TAG_MAP: BTreeMap<UniCase<&'static str>, MachineDataKind> = {
        let mut res = BTreeMap::new();

        res.insert(UniCase::new("json"), MachineDataKind::Json);

        res
    };
}


/// A verified-valid specifier for kinds of machine data.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MachineDataKind {
    Json,
}

impl FromStr for MachineDataKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NAME_TAG_MAP.get(&UniCase::new(s)).map(|&t| t).ok_or_else(|| {
            Error::Parse {
                tp: "machine data specifier",
                wher: "expeced \"json\"".into(),
                more: Some(format!("\"{}\" invalid", s).into()),
            }
        })
    }
}

impl<'de> Deserialize<'de> for MachineDataKind {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        MachineDataKind::from_str(<&'de str>::deserialize(deserializer)?).map_err(|e| {
            let mut buf = vec![];
            e.print_error(&mut buf);
            D::Error::custom(String::from_utf8_lossy(&buf[..buf.len() - 2])) // Drop dot and newline
        })
    }
}

impl fmt::Display for MachineDataKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl Deref for MachineDataKind {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            MachineDataKind::Json => "JSON",
        }
    }
}
