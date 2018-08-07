use self::super::{ScriptElement, StyleElement, LanguageTag};
use toml::de::from_str as from_toml_str;
use std::collections::BTreeMap;
use self::super::super::Error;
use std::default::Default;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;


/// Generic post metadata.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PostMetadata {
    /// Post language override.
    ///
    /// If not present, default post language is used.
    pub language: Option<LanguageTag>,
    /// Post author override.
    ///
    /// If not present, default post author is used.
    pub author: Option<String>,
    /// A set of style descriptors.
    ///
    /// If not present, defaults to empty.
    pub styles: Vec<StyleElement>,
    /// A set of style descriptors.
    ///
    /// If not present, defaults to empty.
    pub scripts: Vec<ScriptElement>,
    /// Additional static data to substitute in header and footer.
    ///
    /// If not present, defaults to empty.
    pub data: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct PostMetadataSerialised {
    pub language: Option<LanguageTag>,
    pub author: Option<String>,
    pub styles: Option<Vec<StyleElement>>,
    pub scripts: Option<Vec<ScriptElement>>,
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
    /// [[scripts]]
    /// class = "link"
    /// data = "/content/assets/syllable.js"
    ///
    /// [[scripts]]
    /// class = "file"
    /// data = "MathJax-config.js"
    ///
    /// [data]
    /// desc = "Każdy koniec to nowy początek [PL]"
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::{ScriptElement, PostMetadata};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # let post_root = temp_dir().join("bloguen-doctest").join("ops-metadata-read_or_default-0");
    /// # fs::create_dir_all(&post_root).unwrap();
    /// # File::create(post_root.join("metadata.toml")).unwrap().write_all(r#"
    /// #     language = "pl"
    /// #
    /// #     [[scripts]]
    /// #     class = "link"
    /// #     data = "/content/assets/syllable.js"
    /// #
    /// #     [[scripts]]
    /// #     class = "file"
    /// #     data = "MathJax-config.js"
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
    ///                language: Some("pl".parse().unwrap()),
    ///                author: None,
    ///                styles: vec![],
    ///                scripts: vec![ScriptElement::from_link("/content/assets/syllable.js"), ScriptElement::from_path("MathJax-config.js")],
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
                    desc: "post metadata".into(),
                    op: "read",
                    more: Some("not UTF-8".into()),
                }
            })?;

        let serialised: PostMetadataSerialised = from_toml_str(&buf).map_err(move |err| {
                Error::FileParsingFailed {
                    desc: "post metadata".into(),
                    errors: Some(err.to_string().into()),
                }
            })?;

        Ok(PostMetadata {
            language: serialised.language,
            author: serialised.author,
            styles: serialised.styles.unwrap_or_default(),
            scripts: serialised.scripts.unwrap_or_default(),
            data: serialised.data.unwrap_or_default(),
        })
    }
}

impl Default for PostMetadata {
    fn default() -> PostMetadata {
        PostMetadata {
            language: None,
            author: None,
            styles: vec![],
            scripts: vec![],
            data: BTreeMap::new(),
        }
    }
}
