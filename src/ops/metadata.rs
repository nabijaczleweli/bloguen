use toml::de::from_str as from_toml_str;
use self::super::super::util::BCP_47;
use std::collections::BTreeMap;
use self::super::super::Error;
use std::default::Default;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;


/// Generic blogue metadata.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PostMetadata {
    /// Post language override.
    ///
    /// If not present, default post language is used.
    pub language: Option<String>,
    /// Additional static data to substitute in header and footer.
    pub data: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct PostMetadataSerialised {
    pub language: Option<String>,
    pub data: Option<BTreeMap<String, String>>,
}

impl PostMetadata {
    /// Read the post metadata from the specified root firectory.
    ///
    /// If the metadata file doesn't exist, `Ok(Default::default())` is returned.
    ///
    /// # Examples
    ///
    /// Given `$POST_ROOT/metadata.toml` containing:
    ///
    /// ```toml
    /// language = "pl"
    ///
    /// [data]
    /// desc = "Każdy koniec to nowy początek [PL]"
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::PostMetadata;
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # let post_root = temp_dir().join("bloguen-doctest").join("ops-metadata-read_or_default-0");
    /// # fs::create_dir_all(&post_root).unwrap();
    /// # File::create(post_root.join("metadata.toml")).unwrap().write_all(r#"
    /// #     language = "pl"
    /// #
    /// #     [data]
    /// #     desc = "Każdy koniec to nowy początek [PL]"
    /// # "#.as_bytes()).unwrap();
    /// # /*
    /// let post_root: PathBuf = /* obtained elsewhere */;
    /// # */
    /// let metadata = PostMetadata::read_or_default(&("$POST_ROOT/".to_string(), post_root.clone())).unwrap();
    /// assert_eq!(metadata,
    ///            PostMetadata {
    ///                language: Some("pl".to_string()),
    ///                data: vec![("desc".to_string(), "Każdy koniec to nowy początek [PL]".to_string())].into_iter().collect(),
    ///            });
    /// ```
    pub fn read_or_default(post_root: &(String, PathBuf)) -> Result<PostMetadata, Error> {
        let mut buf = String::new();
        if let Ok(f) = File::open(post_root.1.join("metadata.toml")) {
                f
            } else {
                return Ok(Default::default());
            }.read_to_string(&mut buf)
            .map_err(|_| {
                Error::Io {
                    desc: "post metadata",
                    op: "read",
                    more: Some("not UTF-8".to_string()),
                }
            })?;

        let mut serialised: PostMetadataSerialised = from_toml_str(&buf).map_err(move |err| {
                Error::FileParsingFailed {
                    desc: "post metadata",
                    errors: Some(err.to_string()),
                }
            })?;

        Ok(PostMetadata {
            language: match serialised.language.as_ref() {
                Some(l) if BCP_47.is_match(l) => Some(String::new()),
                None => None,
                Some(_) => {
                    return Err(Error::Parse {
                        tp: "BCP-47 language tag",
                        wher: "post metadata",
                        more: None,
                    });
                }
            }.map(|_| serialised.language.take().unwrap()),
            data: serialised.data.unwrap_or_default(),
        })
    }
}

impl Default for PostMetadata {
    fn default() -> PostMetadata {
        PostMetadata {
            language: None,
            data: BTreeMap::new(),
        }
    }
}
