use self::super::{MachineDataKind, ScriptElement, StyleElement, LanguageTag};
use self::super::super::util::concat_path;
use toml::de::from_str as from_toml_str;
use std::collections::BTreeMap;
use self::super::super::Error;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;


/// Generic blogue metadata.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlogueDescriptor {
    /// The blogue's display name.
    pub name: String,
    /// The blogue's main author(s).
    ///
    /// Overriden by post metadata, if present.
    ///
    /// If not present, defaults to the current system user's name, which, if not detected, errors out.
    pub author: Option<String>,
    /// Data to put before post HTML, templated.
    ///
    /// Default: `"$ROOT/header.html"`, then `"$ROOT/header.htm"`.
    pub header_file: (String, PathBuf),
    /// Data to put after post HTML, templated.
    ///
    /// Default: `"$ROOT/footer.html"`, then `"$ROOT/footer.htm"`.
    pub footer_file: (String, PathBuf),
    /// Where and which machine datasets to put.
    ///
    /// Each value here is a prefix appended to the output directory under which to put the machine data.
    ///
    /// Values can't be empty (to put machine data at post root use "./").
    pub machine_data: BTreeMap<MachineDataKind, String>,
    /// Default post language.
    ///
    /// Overriden by post metadata, if present.
    ///
    /// If not present, defaults to the current system language, which, if not detected, defaults to en-GB.
    pub language: Option<LanguageTag>,
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
struct BlogueDescriptorSerialised {
    pub name: String,
    pub author: Option<String>,
    pub header: Option<String>,
    pub footer: Option<String>,
    pub machine_data: Option<BTreeMap<MachineDataKind, String>>,
    pub language: Option<LanguageTag>,
    pub styles: Option<Vec<StyleElement>>,
    pub scripts: Option<Vec<ScriptElement>>,
    pub data: Option<BTreeMap<String, String>>,
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
    ///
    /// [[scripts]]
    /// class = "link"
    /// data = "/content/assets/syllable.js"
    ///
    /// [[scripts]]
    /// class = "file"
    /// data = "MathJax-config.js"
    ///
    /// [machine_data]
    /// JSON = "metadata/json/"
    ///
    /// [data]
    /// preferred_system = "capitalism"
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::{BlogueDescriptor, MachineDataKind, ScriptElement};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-descriptor-read");
    /// # fs::create_dir_all(&root).unwrap();
    /// # File::create(root.join("blogue.toml")).unwrap().write_all("\
    /// #     name = \"Блогг\"\n\
    /// #     header = \"head.html\"\n\
    /// #     language = \"pl\"\n\
    /// #     \n\
    /// #     [[scripts]]\n\
    /// #     class = \"link\"\n\
    /// #     data = \"/content/assets/syllable.js\"\n\
    /// #     \n\
    /// #     [[scripts]]\n\
    /// #     class = \"file\"\n\
    /// #     data = \"MathJax-config.js\"\n\
    /// #     \n\
    /// #     [machine_data]\n\
    /// #     JSON = \"metadata/json/\"\n\
    /// #     \n\
    /// #     [data]\n\
    /// #     preferred_system = \"capitalism\"\n\
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
    ///                author: None,
    ///                header_file: ("$ROOT/head.html".to_string(), root.join("head.html")),
    ///                footer_file: ("$ROOT/footer.htm".to_string(), root.join("footer.htm")),
    ///                machine_data: vec![(MachineDataKind::Json, "metadata/json/".to_string())].into_iter().collect(),
    ///                language: Some("pl".parse().unwrap()),
    ///                styles: vec![],
    ///                scripts: vec![ScriptElement::from_link("/content/assets/syllable.js"),
    ///                              ScriptElement::from_path("MathJax-config.js")],
    ///                data: vec![("preferred_system".to_string(),
    ///                            "capitalism".to_string())].into_iter().collect(),
    ///            });
    /// ```
    pub fn read(root: &(String, PathBuf)) -> Result<BlogueDescriptor, Error> {
        let mut buf = String::new();
        File::open(root.1.join("blogue.toml")).map_err(|_| {
                Error::FileNotFound {
                    who: "blogue descriptor",
                    path: format!("{}blogue.toml", root.0).into(),
                }
            })?
            .read_to_string(&mut buf)
            .map_err(|_| {
                Error::Io {
                    desc: "blogue descriptor".into(),
                    op: "read",
                    more: Some("not UTF-8".into()),
                }
            })?;

        let serialised: BlogueDescriptorSerialised = from_toml_str(&buf).map_err(move |err| {
                Error::FileParsingFailed {
                    desc: "blogue descriptor".into(),
                    errors: Some(err.to_string().into()),
                }
            })?;

        let machine_data = serialised.machine_data.unwrap_or_default();
        for (ref k, ref v) in &machine_data {
            if v.find(|c| !['/', '\\'].contains(&c)).is_none() {
                return Err(Error::Parse {
                    tp: "path chunk",
                    wher: "blogue descriptor".into(),
                    more: Some(format!("{} subdir selector empty", k).into()),
                });
            }
        }

        Ok(BlogueDescriptor {
            name: serialised.name,
            author: serialised.author,
            header_file: additional_file(serialised.header, root, "header", "post header")?,
            footer_file: additional_file(serialised.footer, root, "footer", "post footer")?,
            machine_data: machine_data,
            language: serialised.language,
            styles: serialised.styles.unwrap_or_default(),
            scripts: serialised.scripts.unwrap_or_default(),
            data: serialised.data.unwrap_or_default(),
        })
    }
}

fn additional_file(file_opt: Option<String>, root: &(String, PathBuf), tp: &str, error_n: &'static str) -> Result<(String, PathBuf), Error> {
    file_opt.map_or_else(|| {
        check_additional_file(root, &format!("{}.html", tp)).or_else(|| check_additional_file(root, &format!("{}.htm", tp))).ok_or_else(|| {
            Error::FileNotFound {
                who: error_n,
                path: format!("{}{{{1}.html/{1}.htm}}", root.0, tp).into(),
            }
        })
    },
                         |af| {
        let file = concat_path(root.1.clone(), &af);
        let file_s = format!("{}{}", root.0, af);
        if file.exists() {
            if file.is_file() {
                Ok((file_s, file))
            } else {
                Err(Error::WrongFileState {
                    what: "a file",
                    path: file_s.into(),
                })
            }
        } else {
            Err(Error::FileNotFound {
                who: error_n,
                path: file_s.into(),
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
