use toml::de::from_str as from_toml_str;
use self::super::super::Error;
use self::super::LanguageTag;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;


/// Generic blogue metadata.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlogueDescriptor {
    /// The blogue's display name.
    pub name: String,
    /// Data to put before post HTML, templated.
    ///
    /// Default: `"$ROOT/header.html"`, then `"$ROOT/header.htm"`.
    pub header_file: (String, PathBuf),
    /// Data to put after post HTML, templated.
    ///
    /// Default: `"$ROOT/footer.html"`, then `"$ROOT/footer.htm"`.
    pub footer_file: (String, PathBuf),
    /// Default post language.
    ///
    /// Overriden by post metadata, if present.
    ///
    /// If not present, defaults to the current system language, which, if not detected, defaults to en-GB.
    pub language: Option<LanguageTag>,
}

#[derive(Deserialize)]
struct BlogueDescriptorSerialised {
    pub name: String,
    pub header: Option<String>,
    pub footer: Option<String>,
    pub language: Option<LanguageTag>,
}

impl BlogueDescriptor {
    /// Read the blogue descriptor from the specified root firectory.
    ///
    /// # Examples
    ///
    /// Given the following directory layout:
    ///
    /// ```plaintext
    /// $ROOT
    ///   blogue.toml
    ///   head.html
    ///   footer.htm
    /// ```
    ///
    /// Given `$ROOT/blogue.toml` containing:
    ///
    /// ```toml
    /// name = "Блогг"
    /// header = "head.html"
    /// language = "pl"
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::BlogueDescriptor;
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-descriptor-read");
    /// # fs::create_dir_all(&root).unwrap();
    /// # File::create(root.join("blogue.toml")).unwrap().write_all("\
    /// #     name = \"Блогг\"\n\
    /// #     header = \"head.html\"\n\
    /// #     language = \"pl\"\n\
    /// # ".as_bytes()).unwrap();
    /// # File::create(root.join("head.html")).unwrap().write_all("header".as_bytes()).unwrap();
    /// # File::create(root.join("footer.htm")).unwrap().write_all("footer".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    /// let read_tokens = BlogueDescriptor::read(&("$ROOT/".to_string(), root.clone())).unwrap();
    /// assert_eq!(read_tokens,
    ///            BlogueDescriptor {
    ///                name: "Блогг".to_string(),
    ///                header_file: ("$ROOT/head.html".to_string(), root.join("head.html")),
    ///                footer_file: ("$ROOT/footer.htm".to_string(), root.join("footer.htm")),
    ///                language: Some("pl".parse().unwrap()),
    ///            });
    /// ```
    pub fn read(root: &(String, PathBuf)) -> Result<BlogueDescriptor, Error> {
        let mut buf = String::new();
        File::open(root.1.join("blogue.toml")).map_err(|_| {
                Error::FileNotFound {
                    who: "blogue descriptor",
                    path: format!("{}blogue.toml", root.0),
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

        let serialised: BlogueDescriptorSerialised = from_toml_str(&buf).map_err(move |err| {
                Error::FileParsingFailed {
                    desc: "blogue descriptor",
                    errors: Some(err.to_string()),
                }
            })?;

        Ok(BlogueDescriptor {
            name: serialised.name,
            header_file: additional_file(serialised.header, root, "header", "post header")?,
            footer_file: additional_file(serialised.footer, root, "footer", "post footer")?,
            language: serialised.language,
        })
    }
}

fn additional_file(file_opt: Option<String>, root: &(String, PathBuf), tp: &str, error_n: &'static str) -> Result<(String, PathBuf), Error> {
    file_opt.map_or_else(|| {
        check_additional_file(root, &format!("{}.html", tp)).or_else(|| check_additional_file(root, &format!("{}.htm", tp))).ok_or_else(|| {
            Error::FileNotFound {
                who: error_n,
                path: format!("{}{{{1}.html/{1}.htm}}", root.0, tp),
            }
        })
    },
                         |af| {
        let file = af.split(|c| c == '/' || c == '\\').fold(root.1.clone(), |cur, el| cur.join(el));
        let file_s = format!("{}{}", root.0, af);
        if file.exists() {
            if file.is_file() {
                Ok((file_s, file))
            } else {
                Err(Error::WrongFileState {
                    what: "a file",
                    path: file_s,
                })
            }
        } else {
            Err(Error::FileNotFound {
                who: error_n,
                path: file_s,
            })
        }
    })
}

fn check_additional_file(root: &(String, PathBuf), fname: &str) -> Option<(String, PathBuf)> {
    let file = root.1.join(fname);
    if file.is_file() {
        Some((format!("{}{}", root.0, fname), file))
    } else {
        None
    }
}
