use serde::de::{Deserializer, Deserialize, Error as SerdeError};
use serde::ser::{Serializer, Serialize};
use std::collections::BTreeMap;
use self::super::super::Error;
use std::str::FromStr;
use unicase::UniCase;
use std::fmt;


lazy_static! {
    static ref NAME_KIND_MAP: BTreeMap<UniCase<&'static str>, MachineDataKind> = {
        let mut res = BTreeMap::new();

        res.insert(UniCase::new("json"), MachineDataKind::Json);

        res
    };

    static ref ERROR_WHER: String = String::from_utf8(NAME_KIND_MAP.iter()
            .enumerate()
            .map(|(i, v)| (i == NAME_KIND_MAP.len() - 1, v))
            .fold((true, "expected ".as_bytes().to_vec()), |(first, mut acc), (last, (el, _))| {
                if !first {
                    if NAME_KIND_MAP.len() != 2 {
                        acc.extend(b",");
                    }
                    acc.extend(b" ");
                    if last {
                        acc.extend(b"or ");
                    }
                }

                acc.extend(b"\"");
                acc.extend(el.as_bytes());
                acc.extend(b"\"");

                (false, acc)
            })
            .1)
        .unwrap();
}


/// A specifier of machine data format.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MachineDataKind {
    Json,
}

impl MachineDataKind {
    /// Get a kind corresponding to the specified string.
    ///
    /// The string repr of any variant is its name, case-insensitive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::MachineDataKind;
    /// assert_eq!(MachineDataKind::from("json"), Some(MachineDataKind::Json));
    /// assert_eq!(MachineDataKind::from("JsoN"), Some(MachineDataKind::Json));
    /// ```
    pub fn from(s: &str) -> Option<MachineDataKind> {
        NAME_KIND_MAP.get(&UniCase::new(s)).map(|&k| k)
    }

    /// Get a human-readable name of this kind.
    ///
    /// This is re-`from()`able to self.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::MachineDataKind;
    /// assert_eq!(MachineDataKind::Json.name(), "JSON");
    /// ```
    pub fn name(&self) -> &'static str {
        match self {
            MachineDataKind::Json => "JSON",
        }
    }

    /// Get extension to use for saving this kind (without dot).
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::MachineDataKind;
    /// assert_eq!(MachineDataKind::Json.extension(), "json");
    /// ```
    pub fn extension(&self) -> &'static str {
        match self {
            MachineDataKind::Json => "json",
        }
    }
}

impl FromStr for MachineDataKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MachineDataKind::from(s).ok_or_else(|| {
            Error::Parse {
                tp: "machine data specifier",
                wher: (&ERROR_WHER[..]).into(),
                more: format!("\"{}\" invalid", s).into(),
            }
        })
    }
}

impl<'de> Deserialize<'de> for MachineDataKind {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        MachineDataKind::from_str(<&'de str>::deserialize(deserializer)?).map_err(|e| {
            let buf = e.to_string();
            D::Error::custom(&buf[..buf.len() - 1]) // Drop dot
        })
    }
}

impl Serialize for MachineDataKind {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.name())
    }
}

impl fmt::Display for MachineDataKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name().fmt(f)
    }
}
