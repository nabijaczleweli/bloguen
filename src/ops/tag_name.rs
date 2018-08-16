use serde::de::{Deserializer, Deserialize, Error as SerdeError};
use self::super::super::Error;
use std::iter::FromIterator;
use std::path::PathBuf;
use std::str::FromStr;
use std::ops::Deref;
use std::fs::File;
use std::io::Read;
use std::fmt;


/// A verified-valid post tag.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TagName(String);

impl TagName {
    /// Read additional post tags from the specified root directory.
    ///
    /// If the tags file doesn't exist, an empty vector returned.
    ///
    /// # Examples
    ///
    /// Given `$POST_ROOT/tags` containing:
    ///
    /// ```toml
    /// vodka	depression
    ///
    /// коммунизм
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::{ScriptElement, TagName};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # let post_root = temp_dir().join("bloguen-doctest").join("ops-tag_name-load_additional_post_tags");
    /// # fs::create_dir_all(&post_root).unwrap();
    /// # File::create(post_root.join("tags")).unwrap().write_all(r#"
    /// #     vodka	depression
    /// #
    /// #     коммунизм
    /// # "#.as_bytes()).unwrap();
    /// # /*
    /// let post_root: PathBuf = /* obtained elsewhere */;
    /// # */
    /// let tag_names =
    ///     TagName::load_additional_post_tags(&("$POST_ROOT/".to_string(), post_root)).unwrap();
    /// assert_eq!(tag_names,
    ///            vec!["vodka".parse().unwrap(), "depression".parse().unwrap(), "коммунизм".parse().unwrap()]);
    /// ```
    pub fn load_additional_post_tags(post_root: &(String, PathBuf)) -> Result<Vec<TagName>, Error> {
        let mut buf = String::new();
        if let Ok(f) = File::open(post_root.1.join("tags")) {
                f
            } else {
                return Ok(Default::default());
            }.read_to_string(&mut buf)
            .map_err(|_| {
                Error::Io {
                    desc: "additional post tags".into(),
                    op: "read",
                    more: Some("not UTF-8".into()),
                }
            })?;

        Result::from_iter(buf.split(|c: char| c.is_whitespace()).filter(|s| !s.trim().is_empty()).map(TagName::from_str))
    }
}

impl FromStr for TagName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if !s.is_empty() && !s.contains(|c: char| c.is_whitespace() || c.is_control()) {
            Ok(TagName(s.to_string()))
        } else {
            Err(Error::Parse {
                tp: "non-empty WS- and controlless string",
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
