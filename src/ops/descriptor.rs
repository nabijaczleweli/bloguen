use toml::de::from_str as from_toml_str;
use self::super::super::Error;
use std::path::Path;
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
    /// # let tf = temp_dir().join("bloguen-doctest").join("ops-descriptor-read").join("blogue.toml");
    /// # fs::create_dir_all(tf.parent().unwrap()).unwrap();
    /// # fs::File::create(&tf).unwrap().write_all(r#"name = "Блогг""#.as_bytes()).unwrap();
    /// let read_tokens = BlogueDescriptor::read(&tf).unwrap();
    /// assert_eq!(read_tokens, BlogueDescriptor { name: "Блогг".to_string() });
    /// ```
    pub fn read(p: &Path) -> Result<BlogueDescriptor, Error> {
        let mut buf = String::new();
        File::open(p).map_err(|_| {
                Error::FileNotFound {
                    who: "blogue descriptor",
                    path: p.to_path_buf(),
                }
            })?
            .read_to_string(&mut buf)
            .map_err(|_| {
                Error::Io {
                    desc: "blogue descriptor",
                    op: "read",
                    more: Some("not UTF-8"),
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
