use serde::de::{Deserializer, Deserialize, Error as SerdeError};
use serde::ser::{Serializer, Serialize};
use self::super::super::Error;
use bidir_map::BidirMap;
use std::str::FromStr;
use unicase::UniCase;
use std::fmt;


lazy_static! {
    static ref NAME_ORDER_MAP: BidirMap<UniCase<&'static str>, CenterOrder> = bidir_map!{
        UniCase::new("forward") => CenterOrder::Forward,
        UniCase::new("backward") => CenterOrder::Backward,
    };

    static ref ERROR_WHER: String = String::from_utf8(NAME_ORDER_MAP.first_col()
            .enumerate()
            .map(|(i, v)| (i == NAME_ORDER_MAP.len() - 1, v))
            .fold((true, "expected ".as_bytes().to_vec()), |(first, mut acc), (last, el)| {
                if !first {
                    if NAME_ORDER_MAP.len() != 2 {
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


/// A specifier of index centerpiece ordering.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CenterOrder {
    /// Low-to-high
    Forward,
    /// High-to-low
    Backward,
}

impl CenterOrder {
    /// Get a kind corresponding to the specified string.
    ///
    /// The string repr of any variant is its name, case-insensitive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::CenterOrder;
    /// assert_eq!(CenterOrder::from("forwaRd"), Some(CenterOrder::Forward));
    /// assert_eq!(CenterOrder::from("BaCkWard"), Some(CenterOrder::Backward));
    /// ```
    pub fn from(s: &str) -> Option<CenterOrder> {
        NAME_ORDER_MAP.get_by_first(&UniCase::new(s)).map(|&k| k)
    }

    /// Get a human-readable name of this order.
    ///
    /// This is re-`from()`able to self.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::CenterOrder;
    /// assert_eq!(CenterOrder::Forward.name(), "forward");
    /// assert_eq!(CenterOrder::Backward.name(), "backward");
    /// ```
    pub fn name(&self) -> &'static str {
        NAME_ORDER_MAP.get_by_second(&self).unwrap()
    }
}

impl FromStr for CenterOrder {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CenterOrder::from(s).ok_or_else(|| {
            Error::Parse {
                tp: "machine data specifier",
                wher: (&ERROR_WHER[..]).into(),
                more: format!("\"{}\" invalid", s).into(),
            }
        })
    }
}

impl<'de> Deserialize<'de> for CenterOrder {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        CenterOrder::from_str(<&'de str>::deserialize(deserializer)?).map_err(|e| {
            let buf = e.to_string();
            D::Error::custom(&buf[..buf.len() - 1]) // Drop dot
        })
    }
}

impl Serialize for CenterOrder {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.name())
    }
}

impl fmt::Display for CenterOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name().fmt(f)
    }
}

impl Default for CenterOrder {
    fn default() -> CenterOrder {
        CenterOrder::Forward
    }
}