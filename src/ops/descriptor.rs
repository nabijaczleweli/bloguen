use self::super::{MachineDataKind, ScriptElement, StyleElement, CenterOrder, LanguageTag, FeedType};
use std::collections::{BTreeMap, BTreeSet};
use self::super::super::util::concat_path;
use toml::de::from_str as from_toml_str;
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
    /// Subfolder to move assets to, relative to the output root, if present.
    ///
    /// The value is stripped of leading slashes. All backslashes are normalised to forward ones.
    /// The value is ended off with a slash, if not already specified.
    ///
    /// No override is applied if not present – assets are copied alongside the posts' HTML.
    pub asset_dir_override: Option<String>,
    /// Metadata specifying how to generate the blogue index file.
    ///
    /// If not present, index not generated.
    pub index: Option<BlogueDescriptorIndex>,
    /// Where and which machine datasets to put.
    ///
    /// Each value here is a prefix appended to the output directory under which to put the machine data.
    ///
    /// Values can't be empty (to put machine data at post root use "./").
    pub machine_data: BTreeMap<MachineDataKind, String>,
    /// Where and which feeds to put.
    ///
    /// Each value here is a file path appended to the output directory into which to put the machine data.
    pub feeds: BTreeMap<FeedType, String>,
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

/// Metadata pertaining specifically to generating an index file.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlogueDescriptorIndex {
    /// Data to put start index HTML with, templated.
    ///
    /// Default: `"$ROOT/index_header.html"`, then `"$ROOT/index_header.htm"`,
    ///     then `"$ROOT/idx_header.html"`, then `"$ROOT/idx_header.htm"`.
    pub header_file: (String, PathBuf),
    /// Data to put in index HTML for each post, templated.
    ///
    /// Default: `"$ROOT/index_center.html"`, then `"$ROOT/index_center.htm"`,
    ///     then `"$ROOT/idx_center.html"`, then `"$ROOT/idx_center.htm"`.
    pub center_file: (String, PathBuf),
    /// Data to put to end index HTML with, templated.
    ///
    /// Default: `"$ROOT/index_footer.html"`, then `"$ROOT/index_footer.htm"`,
    ///     then `"$ROOT/idx_footer.html"`, then `"$ROOT/idx_footer.htm"`.
    pub footer_file: (String, PathBuf),
    /// The order to put center templates in.
    ///
    /// If not present, defaults to forward.
    pub center_order: CenterOrder,
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
    pub asset_dir: Option<String>,
    pub index: Option<BlogueDescriptorIndexSerialised>,
    pub machine_data: Option<BTreeMap<MachineDataKind, String>>,
    pub feeds: Option<BTreeMap<FeedType, String>>,
    pub language: Option<LanguageTag>,
    pub styles: Option<Vec<StyleElement>>,
    pub scripts: Option<Vec<ScriptElement>>,
    pub data: Option<BTreeMap<String, String>>,
}

#[derive(Deserialize)]
struct BlogueDescriptorIndexSerialised {
    pub generate: Option<bool>,
    pub header: Option<String>,
    pub center: Option<String>,
    pub footer: Option<String>,
    pub order: Option<CenterOrder>,
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
    ///   idx_head.html
    ///   центр.html
    ///   index_footer.htm
    /// ```
    ///
    /// Given `$ROOT/blogue.toml` containing:
    ///
    /// ```toml
    /// name = "Блогг"
    /// header = "head.html"
    /// language = "pl"
    /// asset_dir = "assets"
    ///
    /// [index]
    /// header = "idx_head.html"
    /// center = "центр.html"
    /// order = "backward"
    /// styles = ["literal:.indented { text-indent: 1em; }"]
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
    /// [feeds]
    /// RSS = "feed.rss"
    /// ATOM = "feed.atom"
    ///
    /// [data]
    /// preferred_system = "capitalism"
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::{BlogueDescriptorIndex, BlogueDescriptor, MachineDataKind, ScriptElement, StyleElement,
    /// #                    CenterOrder, FeedType};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-descriptor-read");
    /// # fs::create_dir_all(&root).unwrap();
    /// # File::create(root.join("blogue.toml")).unwrap().write_all("\
    /// #     name = \"Блогг\"\n\
    /// #     header = \"head.html\"\n\
    /// #     language = \"pl\"\n\
    /// #     asset_dir = \"assets\"\n\
    /// #     \n\
    /// #     [index]\n\
    /// #     header = \"idx_head.html\"\n\
    /// #     center = \"центр.html\"\n\
    /// #     order = \"backward\"\n\
    /// #     styles = [\"literal:.indented { text-indent: 1em; }\"]\n\
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
    /// #     [feeds]\n\
    /// #     RSS = \"feed.rss\"\n\
    /// #     Atom = \"feed.atom\"\n\
    /// #     \n\
    /// #     [data]\n\
    /// #     preferred_system = \"capitalism\"\n\
    /// # ".as_bytes()).unwrap();
    /// # File::create(root.join("head.html")).unwrap().write_all("header".as_bytes()).unwrap();
    /// # File::create(root.join("footer.htm")).unwrap().write_all("footer".as_bytes()).unwrap();
    /// # File::create(root.join("idx_head.html")).unwrap().write_all("index header".as_bytes()).unwrap();
    /// # File::create(root.join("центр.html")).unwrap().write_all("index центр".as_bytes()).unwrap();
    /// # File::create(root.join("index_footer.htm")).unwrap().write_all("index footer".as_bytes()).unwrap();
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
    ///                asset_dir_override: Some("assets/".to_string()),
    ///                machine_data: vec![(MachineDataKind::Json, "metadata/json/".to_string())].into_iter().collect(),
    ///                feeds: vec![(FeedType::Rss, "feed.rss".to_string()),
    ///                            (FeedType::Atom, "feed.atom".to_string())].into_iter().collect(),
    ///                language: Some("pl".parse().unwrap()),
    ///                styles: vec![],
    ///                scripts: vec![ScriptElement::from_link("/content/assets/syllable.js"),
    ///                              ScriptElement::from_path("MathJax-config.js")],
    ///                index: Some(BlogueDescriptorIndex {
    ///                    header_file: ("$ROOT/idx_head.html".to_string(), root.join("idx_head.html")),
    ///                    center_file: ("$ROOT/центр.html".to_string(), root.join("центр.html")),
    ///                    footer_file: ("$ROOT/index_footer.htm".to_string(), root.join("index_footer.htm")),
    ///                    center_order: CenterOrder::Backward,
    ///                    styles: vec![StyleElement::from_literal(".indented { text-indent: 1em; }")],
    ///                    scripts: vec![],
    ///                    data: vec![].into_iter().collect(),
    ///                }),
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
                    more: "not UTF-8".into(),
                }
            })?;

        let serialised: BlogueDescriptorSerialised = from_toml_str(&buf).map_err(move |err| {
                Error::FileParsingFailed {
                    desc: "blogue descriptor".into(),
                    errors: err.to_string().into(),
                }
            })?;

        let asset_dir_override = serialised.asset_dir.map(|mut ad| {
            if let Some(i) = ad.find(|c| !['/', '\\'].contains(&c)) {
                ad.replace_range(..i, "");
            }

            let mut last_slash = 0;
            while let Some(backslash) = ad[last_slash..].find('\\') {
                ad.replace_range(last_slash + backslash..last_slash + backslash + 1, "/");
                last_slash += backslash + 1;
            }

            if !ad.ends_with('/') {
                ad.push('/');
            }

            ad
        });

        let machine_data = serialised.machine_data.unwrap_or_default();
        for (ref k, ref v) in &machine_data {
            if v.find(|c| !['/', '\\'].contains(&c)).is_none() {
                return Err(Error::Parse {
                    tp: "path chunk",
                    wher: "blogue descriptor".into(),
                    more: format!("{} subdir selector empty", k).into(),
                });
            }
        }

        let feeds = serialised.feeds.unwrap_or_default();
        for (ref k, ref v) in &feeds {
            let more = if v.is_empty() {
                Some(format!("{} filename empty", k))
            } else if v.ends_with(|c| ['/', '\\'].contains(&c)) {
                Some(format!("{} filename {:?} ends with path separator", k, v))
            } else {
                None
            };

            if let Some(more) = more {
                return Err(Error::Parse {
                    tp: "path chunk",
                    wher: "blogue descriptor".into(),
                    more: more.into(),
                });
            }
        }
        {
            let mut feeds_fnames = BTreeSet::new();
            for v in feeds.values() {
                if !feeds_fnames.insert(v) {
                    return Err(Error::Parse {
                        tp: "path chunk",
                        wher: "blogue descriptor".into(),
                        more: format!("feed filename {:?} duplicate", v).into(),
                    });
                }
            }
        }

        Ok(BlogueDescriptor {
            name: serialised.name,
            author: serialised.author,
            header_file: additional_file(serialised.header, root, "header", "post header")?,
            footer_file: additional_file(serialised.footer, root, "footer", "post footer")?,
            asset_dir_override: asset_dir_override,
            index: match serialised.index {
                Some(mut si) => {
                    match si.generate {
                        None | Some(true) => {
                                Some(BlogueDescriptorIndex {
                                    header_file: additional_file(si.header.clone(), root, "index_header", "index header")
                                                     .or_else(|_| additional_file(si.header.take(), root, "idx_header", "index header"))?,
                                    center_file: additional_file(si.center.clone(), root, "index_center", "index center")
                                                     .or_else(|_| additional_file(si.center.take(), root, "idx_center", "index center"))?,
                                    footer_file: additional_file(si.footer.clone(), root, "index_footer", "index footer")
                                                     .or_else(|_| additional_file(si.footer.take(), root, "idx_footer", "index footer"))?,
                                    center_order: si.order.unwrap_or_default(),
                                    styles: si.styles.unwrap_or_default(),
                                    scripts: si.scripts.unwrap_or_default(),
                                    data: si.data.unwrap_or_default(),
                                })
                            }
                        Some(false) => None,
                    }
                }
                None => None,
            },
            machine_data: machine_data,
            feeds: feeds,
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
