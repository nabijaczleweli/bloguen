use toml::de::from_str as from_toml_str;
use self::super::super::Error;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;


/// Generic blogue metadata.
#[derive(Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlogueDescriptor {
    /// The blogue's display name.
    pub name: String,
}

impl BlogueDescriptor {
    /// Read all queued tweets from the specified file.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::BlogueDescriptor;
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # use std::fs;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-descriptor-read");
    /// # fs::create_dir_all(&root).unwrap();
    /// # fs::File::create(root.join("blogue.toml")).unwrap().write_all(r#"name = "Блогг""#.as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    /// let read_tokens = BlogueDescriptor::read(&("$ROOT/blogue.toml".to_string(),
    ///                                            root.join("blogue.toml"))).unwrap();
    /// assert_eq!(read_tokens, BlogueDescriptor { name: "Блогг".to_string() });
    /// ```
    pub fn read(p: &(String, PathBuf)) -> Result<BlogueDescriptor, Error> {
        let mut buf = String::new();
        File::open(&p.1).map_err(|_| {
                Error::FileNotFound {
                    who: "blogue descriptor",
                    path: p.0.clone(),
                }
            })?
            .read_to_string(&mut buf)
            .map_err(|_| {
                Error::Io {
                    desc: "blogue descriptor",
                    op: "read",
                    more: Some("not UTF-8".to_string()),
                }
            })?;

        from_toml_str(&buf).map_err(move |err| {
            Error::FileParsingFailed {
                desc: "blogue descriptor",
                errors: Some(err.to_string()),
            }
        })
    }
}
