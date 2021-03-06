use self::super::{MachineDataKind, ScriptElement, StyleElement, LanguageTag, FeedType, TagName, feed_type_post_footer, feed_type_post_header,
                  machine_output_kind, format_output};
use self::super::super::util::{PolyWrite, MARKDOWN_OPTIONS, extract_actual_assets, name_based_post_time, extract_links, concat_path, path_depth, read_file,
                               mul_str};
use walkdir::{Error as WalkDirError, DirEntry, WalkDir};
use chrono::{NaiveTime, DateTime, TimeZone};
use chrono::offset::Local as LocalOffset;
use comrak::{self, Arena as ComrakArena};
use std::io::{Error as IoError, Write};
use std::collections::BTreeMap;
use self::super::super::Error;
use std::num::ParseIntError;
use std::iter::FromIterator;
use std::fs::{self, File};
use std::path::PathBuf;
use regex::Regex;
use std::str;


lazy_static! {
    static ref POST_DIR_NAME: Regex = Regex::new(include_str!("../../assets/post_dir_name.regex").trim()).unwrap();
}


/// Information about a blogue post.
///
/// Use `list()` to find valid post directories, then use `new()` to get the post data.
///
/// A correct post directory name is `#+. YYYY-MM-DD [HH-MM[-SS]] name`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BloguePost {
    /// Directory containing the post data.
    pub source_dir: (String, PathBuf),
    /// Post number.
    pub number: (usize, String),
    /// Post name.
    pub name: String,
    /// Date & time of posting.
    pub datetime: DateTime<LocalOffset>,
}

impl BloguePost {
    /// List correct post directories in the specified directory.
    ///
    /// A correct post directory name is `#+. YYYY-MM-DD [HH-MM[-SS]] name`.
    ///
    /// Examples:
    ///
    /// For the following directory tree:
    ///
    /// ```plaintext
    /// posts/
    ///   temp/
    ///   001. 2018-01-08 16-52 The venture into crocheting/
    ///   002. 2018-01-08 acquiescence.md
    ///   003. 2018-02-05 release-front - release front-end/
    ///   004. stir plate/
    ///   blogue.toml
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::BloguePost;
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-post-list");
    /// # let _ = fs::remove_dir_all(&root);
    /// # for d in &["temp",
    /// #           "001. 2018-01-08 16-52 The venture into crocheting",
    /// #           "003. 2018-02-05 release-front - release front-end",
    /// #           "004. stir plate"] {
    /// #   fs::create_dir_all(root.join("posts").join(d)).unwrap();
    /// # }
    /// # for f in &["002. 2018-01-08 acquiescence.md", "blogue.toml"] {
    /// #   File::create(root.join("posts").join(f)).unwrap();
    /// # }
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    /// let posts = BloguePost::list(&("$ROOT/posts/".to_string(), root.join("posts"))).unwrap();
    /// assert_eq!(&posts[..],
    ///     &[("$ROOT/posts/001. 2018-01-08 16-52 The venture into crocheting/".to_string(),
    ///        root.join("posts").join("001. 2018-01-08 16-52 The venture into crocheting")),
    ///       ("$ROOT/posts/003. 2018-02-05 release-front - release front-end/".to_string(),
    ///        root.join("posts").join("003. 2018-02-05 release-front - release front-end"))][..]);
    /// ```
    pub fn list(within: &(String, PathBuf)) -> Result<Vec<(String, PathBuf)>, Error> {
        Ok(Result::<Vec<DirEntry>, WalkDirError>::from_iter(WalkDir::new(&within.1).sort_by(|lhs, rhs| lhs.file_name().cmp(rhs.file_name())).into_iter())
            .map_err(|e| {
                Error::Io {
                    desc: "post list".into(),
                    op: "list",
                    more: e.to_string().into(),
                }
            })?
            .into_iter()
            .filter(|e| e.file_name().to_str().map(|fname| POST_DIR_NAME.is_match(fname)).unwrap_or(false) && e.path().is_dir())
            .map(|e| (format!("{}{}/", within.0, e.file_name().to_str().unwrap()), e.path().to_path_buf()))
            .collect::<Vec<_>>())
    }

    /// Read post data into a `BloguePost` instance.
    ///
    /// Examples:
    ///
    /// ```
    /// # extern crate bloguen;
    /// # extern crate chrono;
    /// # use chrono::offset::Local as LocalOffset;
    /// # use bloguen::ops::BloguePost;
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use chrono::TimeZone;
    /// # use bloguen::Error;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-post-new");
    /// # let _ = fs::remove_dir_all(&root);
    /// # for d in &["01. 2018-01-08 16-52 The venture into crocheting",
    /// #            "003. 2018-02-05 release-front - release front-end",
    /// #            "004. stir plate"] {
    /// #   fs::create_dir_all(root.join("posts").join(d)).unwrap();
    /// # }
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    ///
    /// let dir = ("$ROOT/posts/01. 2018-01-08 16-52 The venture into crocheting".to_string(),
    ///            root.join("posts").join("01. 2018-01-08 16-52 The venture into crocheting"));
    /// assert_eq!(BloguePost::new(dir.clone()),
    ///            Ok(BloguePost {
    ///                source_dir: dir,
    ///                number: (1, "01".to_string()),
    ///                name: "The venture into crocheting".to_string(),
    ///                datetime: LocalOffset.ymd(2018, 01, 08).and_hms(16, 52, 00),
    ///            }));
    ///
    /// let dir = ("$ROOT/posts/003. 2018-02-05 release-front - release front-end".to_string(),
    ///            root.join("posts").join("003. 2018-02-05 release-front - release front-end"));
    /// assert_eq!(BloguePost::new(dir.clone()),
    ///            Ok(BloguePost {
    ///                source_dir: dir,
    ///                number: (3, "003".to_string()),
    ///                name: "release-front - release front-end".to_string(),
    ///                datetime: LocalOffset.ymd(2018, 02, 05).and_hms(23, 24, 43),
    ///            }));
    ///
    /// let dir = ("$ROOT/posts/004. stir plate".to_string(),
    ///            root.join("posts").join("004. stir plate"));
    /// assert_eq!(BloguePost::new(dir.clone()),
    ///            Err(Error::Parse {
    ///                tp: "post directory filename",
    ///                wher: "blogue post".into(),
    ///                more: "not found".into(),
    ///            }));
    /// ```
    pub fn new(wher: (String, PathBuf)) -> Result<BloguePost, Error> {
        fn uint_err(wher: &'static str, err: ParseIntError) -> Error {
            Error::Parse {
                tp: "unsigned int",
                wher: wher.into(),
                more: err.to_string().into(),
            }
        }


        let mut ret = {
            let mch = POST_DIR_NAME.captures(wher.1.file_name().unwrap().to_str().unwrap())
                .ok_or_else(|| {
                    Error::Parse {
                        tp: "post directory filename",
                        wher: "blogue post".into(),
                        more: "not found".into(),
                    }
                })?;
            let name = mch.name("name").unwrap().as_str();
            let number = mch.name("post_number").unwrap().as_str();

            BloguePost {
                source_dir: (String::new(), PathBuf::from(String::new())),
                number: (number.parse().map_err(|e| uint_err("post number", e))?, number.to_string()),
                name: name.to_string(),
                datetime: LocalOffset.ymd(mch.name("date_year").unwrap().as_str().parse().map_err(|e| uint_err("post date year", e))?,
                         mch.name("date_month").unwrap().as_str().parse().map_err(|e| uint_err("post date month", e))?,
                         mch.name("date_day").unwrap().as_str().parse().map_err(|e| uint_err("post date day", e))?)
                    .and_time(if let Some(hour) = mch.name("time_hour") {
                        NaiveTime::from_hms(hour.as_str().parse().map_err(|e| uint_err("post time hour", e))?,
                                            mch.name("time_minute").unwrap().as_str().parse().map_err(|e| uint_err("post time minute", e))?,
                                            if let Some(second) = mch.name("time_second") {
                                                second.as_str().parse().map_err(|e| uint_err("post time second", e))?
                                            } else {
                                                0
                                            })
                    } else {
                        name_based_post_time(name)
                    })
                    .unwrap(),
            }
        };
        ret.source_dir = wher;
        Ok(ret)
    }

    /// Generate an HTML output from the post into the specified output directory.
    ///
    /// Alternate output is filled with the HTML-formatted post Markdown.
    ///
    /// Center output is filled with the specified template filled-out with additional `post_content` data element
    /// consisting of the HTML-formatted post Markdown.
    ///
    /// Returns: set of links in the markdown source.
    ///
    /// # Examples
    ///
    /// Given the following:
    ///
    /// ```plaintext
    /// src/
    ///   01. 2018-01-08 16-52 The venture into crocheting/
    ///     post.md
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::util::LANGUAGE_EN_GB;
    /// # use bloguen::ops::BloguePost;
    /// # use std::io::{Write, Read};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-post-generate");
    /// # let _ = fs::remove_dir_all(&root);
    /// # fs::create_dir_all(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")).unwrap();
    /// # File::create(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")
    /// #                  .join("post.md")).unwrap().write_all("[Блогг](url.html)".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    /// let post =
    ///     BloguePost::new(("$ROOT/src/01. 2018-01-08 16-52 The venture into crocheting".to_string(),
    ///         root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting"))).unwrap();
    /// assert!(post.generate(&("$ROOT/out/".to_string(), root.join("out")), None, None, None, "header", "footer",
    ///                       "Блогг", &LANGUAGE_EN_GB, "autheur", &[], &[], &Default::default(), &Default::default(),
    ///                       &[], &[], &[], &[]).is_ok());
    /// # assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out")), None, None, None, "header", "footer",
    /// #                          "Блогг", &LANGUAGE_EN_GB, "autheur", &[], &[], &Default::default(), &Default::default(),
    /// #                          &[], &[], &[], &[]),
    /// #            Ok(vec!["url.html".to_string()]));
    ///
    /// assert!(root.join("out").join("posts")
    ///             .join("01. 2018-01-08 16-52-00 The venture into crocheting.html").is_file());
    /// # let mut read = String::new();
    /// # File::open(root.join("out").join("posts").join("01. 2018-01-08 16-52-00 The venture into crocheting.html"))
    /// #                .unwrap().read_to_string(&mut read).unwrap();
    /// # assert_eq!(read, "header<p><a href=\"url.html\">Блогг</a></p>\nfooter");
    /// ```
    pub fn generate(&self, into: &(String, PathBuf), mut alt_output: Option<&mut dyn Write>, center_output: Option<(&str, &mut dyn Write)>,
                    asset_override: Option<&str>, post_header: &str, post_footer: &str, blog_name: &str, language: &LanguageTag, author: &str,
                    spec_tags: &[TagName], free_tags: &[TagName], post_data: &BTreeMap<String, String>, global_data: &BTreeMap<String, String>,
                    post_styles: &[StyleElement], global_styles: &[StyleElement], post_scripts: &[ScriptElement], global_scripts: &[ScriptElement])
                    -> Result<Vec<String>, Error> {
        fn write_err(err: IoError, desc: &'static str) -> Error {
            Error::Io {
                desc: desc.into(),
                op: "write",
                more: err.to_string().into(),
            }
        }


        let post_text = read_file(&(format!("{}post.md", self.source_dir.0), self.source_dir.1.join("post.md")), "post text")?;

        let arena = ComrakArena::new();
        let root = comrak::parse_document(&arena, &post_text, &MARKDOWN_OPTIONS);
        let out_links = extract_links(root)?;

        fs::create_dir_all(into.1.join("posts")).map_err(|e| {
                Error::Io {
                    desc: "posts directory".into(),
                    op: "create",
                    more: e.to_string().into(),
                }
            })?;

        let normalised_name = self.normalised_name();
        let post_html_path = into.1.join("posts").join(format!("{}.html", normalised_name));
        let mut post_html_f = File::create(post_html_path).map_err(|e| {
                Error::Io {
                    desc: "post HTML".into(),
                    op: "create",
                    more: e.to_string().into(),
                }
            })?;

        let original_name = self.source_dir.1.file_name().unwrap().to_str().unwrap();
        let normalised_name_err = format_output(post_header,
                                                blog_name,
                                                language,
                                                &[global_data, post_data],
                                                &original_name,
                                                &normalised_name,
                                                self.number.0,
                                                &self.name,
                                                author,
                                                &self.datetime,
                                                &[spec_tags, free_tags],
                                                &[global_styles, post_styles],
                                                &[global_scripts, post_scripts],
                                                &mut post_html_f,
                                                normalised_name.clone())?;

        let mut center_output = center_output.map(|(f, o)| (f, o, vec![]));
        if let Some(asset_override) = asset_override {
            let mut asset_set = extract_actual_assets(&self.source_dir.1, root)?;

            asset_set.iter_mut().for_each(|url| { url.splice(0..0, asset_override.as_bytes().iter().cloned()); });
            if let Some((_, _, center_tmp)) = center_output.as_mut() {
                comrak::format_html(root, &MARKDOWN_OPTIONS, center_tmp).map_err(|e| write_err(e, "post center HTML"))?;
            }

            asset_set.iter_mut().for_each(|url| { url.splice(0..0, b"../".iter().cloned()); });
            if let Some(ref mut alt_out) = alt_output.as_mut() {
                    comrak::format_html(root, &MARKDOWN_OPTIONS, &mut PolyWrite(&mut post_html_f, alt_out))
                } else {
                    comrak::format_html(root, &MARKDOWN_OPTIONS, &mut post_html_f)
                }.map_err(|e| write_err(e, "post HTML"))?;
        } else {
            if let Some(ref mut alt_out) = alt_output.as_mut() {
                    comrak::format_html(root, &MARKDOWN_OPTIONS, &mut PolyWrite(&mut post_html_f, alt_out))
                } else {
                    comrak::format_html(root, &MARKDOWN_OPTIONS, &mut post_html_f)
                }.map_err(|e| write_err(e, "post HTML"))?;

            if let Some((_, _, center_tmp)) = center_output.as_mut() {
                let mut asset_set = extract_actual_assets(&self.source_dir.1, root)?;
                asset_set.iter_mut().for_each(|url| { url.splice(0..0, b"posts/".iter().cloned()); });

                comrak::format_html(root, &MARKDOWN_OPTIONS, center_tmp).map_err(|e| write_err(e, "post center HTML"))?;
            }
        }

        let normalised_name_err = format_output(post_footer,
                                                blog_name,
                                                language,
                                                &[global_data, post_data],
                                                &original_name,
                                                &normalised_name,
                                                self.number.0,
                                                &self.name,
                                                author,
                                                &self.datetime,
                                                &[spec_tags, free_tags],
                                                &[global_styles, post_styles],
                                                &[global_scripts, post_scripts],
                                                &mut post_html_f,
                                                normalised_name_err)?;

        if let Some((center, mut center_out, center_temp)) = center_output {
            let mut temp_data = BTreeMap::new();
            temp_data.insert("post_content".to_string(),
                             String::from_utf8(center_temp).map_err(|e| {
                    Error::Parse {
                        tp: "UTF-8 string",
                        wher: "index file post metadata".into(),
                        more: e.to_string().into(),
                    }
                })?);

            format_output(center,
                          blog_name,
                          language,
                          &[global_data, post_data, &temp_data],
                          &original_name,
                          &normalised_name,
                          self.number.0,
                          &self.name,
                          author,
                          &self.datetime,
                          &[spec_tags, free_tags],
                          &[global_styles, post_styles],
                          &[global_scripts, post_scripts],
                          &mut center_out,
                          normalised_name_err)?;
        }

        Ok(out_links)
    }

    /// Generate machine output of the specified kind from the post into the specified subpath in the specified output
    /// directory.
    ///
    /// # Examples
    ///
    /// Given the following:
    ///
    /// ```plaintext
    /// src/
    ///   01. 2018-01-08 16-52 The venture into crocheting/
    ///     post.md
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::{MachineDataKind, BloguePost};
    /// # use bloguen::util::LANGUAGE_EN_GB;
    /// # use std::io::{Write, Read};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-post-create_machine_output");
    /// # let _ = fs::remove_dir_all(&root);
    /// # fs::create_dir_all(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")).unwrap();
    /// # File::create(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")
    /// #                  .join("post.md")).unwrap().write_all("[Блогг](url.html)".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    /// let post =
    ///     BloguePost::new(("$ROOT/src/01. 2018-01-08 16-52 The venture into crocheting".to_string(),
    ///         root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting"))).unwrap();
    /// let machine_output_file =
    ///     post.create_machine_output(&("$ROOT/out/".to_string(), root.join("out")), "machine/",
    ///                                &MachineDataKind::Json).unwrap();
    ///
    /// assert!(root.join("out").join("machine")
    ///             .join("01. 2018-01-08 16-52-00 The venture into crocheting.json").is_file());
    /// ```
    pub fn create_machine_output(&self, into: &(String, PathBuf), subpath: &str, kind: &MachineDataKind) -> Result<File, Error> {
        let mut machine_root_path = concat_path(&into.1, subpath);
        fs::create_dir_all(&machine_root_path).map_err(|e| {
                Error::Io {
                    desc: format!("{} directory", subpath).into(),
                    op: "create",
                    more: e.to_string().into(),
                }
            })?;

        machine_root_path.push(format!("{}.{}", self.normalised_name(), kind.extension()));
        let post_kind_f = File::create(machine_root_path).map_err(|e| {
                Error::Io {
                    desc: format!("post {}", kind).into(),
                    op: "create",
                    more: e.to_string().into(),
                }
            })?;

        Ok(post_kind_f)
    }

    /// Generate machine output of the specified kind from the post into the specified subpath in the specified output
    /// directory.
    ///
    /// # Examples
    ///
    /// Given the following:
    ///
    /// ```plaintext
    /// src/
    ///   01. 2018-01-08 16-52 The venture into crocheting/
    ///     post.md
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::{MachineDataKind, BloguePost};
    /// # use bloguen::util::LANGUAGE_EN_GB;
    /// # use std::io::{Write, Read};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::str;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-post-generate_machine");
    /// # let _ = fs::remove_dir_all(&root);
    /// # fs::create_dir_all(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")).unwrap();
    /// # File::create(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")
    /// #                  .join("post.md")).unwrap().write_all("[Блогг](url.html)".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    /// let post =
    ///     BloguePost::new(("$ROOT/src/01. 2018-01-08 16-52 The venture into crocheting".to_string(),
    ///         root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting"))).unwrap();
    ///
    /// let mut out = vec![];
    /// assert!(post.generate_machine(&mut out, &MachineDataKind::Json,
    ///                               "Блогг", &LANGUAGE_EN_GB, "autheur", &[], &[], &Default::default(), &Default::default(),
    ///                               &[], &[], &[], &[]).is_ok());
    ///
    /// assert!(!out.is_empty());
    /// assert!(str::from_utf8(&out).unwrap().contains("The venture into crocheting"));  // &c.
    /// # assert!(out.starts_with(r##"{
    /// #     "number": 1,
    /// #     "language": "en-GB",
    /// #     "title": "The venture into crocheting",
    /// #     "author": "autheur",
    /// #
    /// #     "raw_post_name": "01. 2018-01-08 16-52 The venture into crocheting",
    /// #     "blog_name": "Блогг","##.as_bytes()));
    /// # assert!(out.ends_with(r##"
    /// #     "tags": [
    /// #     ],
    /// #     "additional_data": {
    /// #     },
    /// #
    /// #     "styles": [
    /// #     ],
    /// #     "scripts": [
    /// #     ],
    /// #
    /// #     "bloguen-version": "0.1.1"
    /// # }"##.as_bytes()));
    /// ```
    pub fn generate_machine<T: Write>(&self, into: &mut T, kind: &MachineDataKind, blog_name: &str, language: &LanguageTag, author: &str,
                                      spec_tags: &[TagName], free_tags: &[TagName], post_data: &BTreeMap<String, String>,
                                      global_data: &BTreeMap<String, String>, post_styles: &[StyleElement], global_styles: &[StyleElement],
                                      post_scripts: &[ScriptElement], global_scripts: &[ScriptElement])
                                      -> Result<(), Error> {
        let original_name = self.source_dir.1.file_name().unwrap().to_str().unwrap();
        machine_output_kind(kind)(blog_name,
                                  language,
                                  &[global_data, post_data],
                                  &original_name,
                                  self.number.0,
                                  &self.name,
                                  author,
                                  &self.datetime,
                                  &[spec_tags, free_tags],
                                  &[global_styles, post_styles],
                                  &[global_scripts, post_scripts],
                                  into,
                                  self.normalised_name())?;

        Ok(())
    }

    /// Generate header for this post of the specified feed type.
    ///
    /// # Examples
    ///
    /// Given the following:
    ///
    /// ```plaintext
    /// src/
    ///   01. 2018-01-08 16-52 The venture into crocheting/
    ///     post.md
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # extern crate bloguen;
    /// # extern crate chrono;
    /// # use bloguen::ops::{FeedType, BloguePost};
    /// # use bloguen::util::LANGUAGE_EN_GB;
    /// # use chrono::offset::Local;
    /// # use std::io::{Write, Read};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::str;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-post-generate_feed_head");
    /// # let _ = fs::remove_dir_all(&root);
    /// # fs::create_dir_all(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")).unwrap();
    /// # File::create(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")
    /// #                  .join("post.md")).unwrap().write_all("[Блогг](url.html)".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    /// let post =
    ///     BloguePost::new(("$ROOT/src/01. 2018-01-08 16-52 The venture into crocheting".to_string(),
    ///         root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting"))).unwrap();
    ///
    /// let mut out = vec![];
    /// assert!(post.generate_feed_head(&mut out, &FeedType::Rss, "feeds/rss.xml",
    ///                                           &LANGUAGE_EN_GB, "nabijaczleweli").is_ok());
    ///
    /// let out = String::from_utf8(out).unwrap();
    /// # let mut pubdate_local_rfc2822 = out.lines().find(|l| l.contains("pubDate")).unwrap();
    /// # pubdate_local_rfc2822 = &pubdate_local_rfc2822[6   + 1 + 7 + 1..pubdate_local_rfc2822.len() - (1 + 7 + 1 + 1)];
    /// # /*
    /// let pubdate_local_rfc2822 = /* extracted from output's pubDate tag */;
    /// # */
    /// assert_eq!(out, format!(r###"
    ///     <item>
    ///       <title>The venture into crocheting</title>
    ///       <author>nabijaczleweli</author>
    ///       <link>../posts/01. 2018-01-08 16-52-00 The venture into crocheting.html</link>
    ///       <pubDate>{}</pubDate>
    ///       <guid>01. 2018-01-08 16-52-00 The venture into crocheting</guid>
    ///       <description>
    /// "###, pubdate_local_rfc2822));
    /// ```
    pub fn generate_feed_head<T: Write>(&self, into: &mut T, tp: &FeedType, fname: &str, language: &LanguageTag, author: &str) -> Result<(), Error> {
        let norm_name = self.normalised_name();

        let depth = path_depth(fname);
        let link_pref = if depth - 1 > 0 {
            mul_str("../", depth as usize - 1)
        } else {
            String::new()
        };
        let link = format!("{}posts/{}.html", link_pref, norm_name);

        feed_type_post_header(tp)(&self.name,
                                  &norm_name,
                                  language,
                                  author,
                                  &link[..link_pref.len() + 5 + 1],
                                  &link,
                                  &self.datetime,
                                  into,
                                  self.normalised_name())?;

        Ok(())
    }

    /// Generate footer for this post of the specified feed type.
    ///
    /// # Examples
    ///
    /// Given the following:
    ///
    /// ```plaintext
    /// src/
    ///   01. 2018-01-08 16-52 The venture into crocheting/
    ///     post.md
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::{FeedType, BloguePost};
    /// # use bloguen::util::LANGUAGE_EN_GB;
    /// # use std::io::{Write, Read};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::str;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-post-generate_feed_foot");
    /// # let _ = fs::remove_dir_all(&root);
    /// # fs::create_dir_all(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")).unwrap();
    /// # File::create(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")
    /// #                  .join("post.md")).unwrap().write_all("[Блогг](url.html)".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    /// let post =
    ///     BloguePost::new(("$ROOT/src/01. 2018-01-08 16-52 The venture into crocheting".to_string(),
    ///         root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting"))).unwrap();
    ///
    /// let mut out = vec![];
    /// assert!(post.generate_feed_foot(&mut out, &FeedType::Rss).is_ok());
    ///
    /// assert_eq!(String::from_utf8(out).unwrap(), r###"      </description>
    ///     </item>
    /// "###);
    /// ```
    pub fn generate_feed_foot<T: Write>(&self, into: &mut T, tp: &FeedType) -> Result<(), Error> {
        feed_type_post_footer(tp)(into, self.normalised_name())?;

        Ok(())
    }

    /// Get a normalised output name for this post.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::BloguePost;
    /// # use std::io::{Write, Read};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-post-normalised_name");
    /// # let _ = fs::remove_dir_all(&root);
    /// # fs::create_dir_all(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")).unwrap();
    /// # File::create(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")
    /// #                  .join("post.md")).unwrap().write_all("Блогг".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    /// let post =
    ///     BloguePost::new(("$ROOT/src/01. 2018-01-08 16-52 The venture into crocheting".to_string(),
    ///         root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting"))).unwrap();
    /// assert_eq!(post.normalised_name(), "01. 2018-01-08 16-52-00 The venture into crocheting");
    /// ```
    pub fn normalised_name(&self) -> String {
        format!("{}. {} {}", self.number.1, self.datetime.format("%Y-%m-%d %H-%M-%S"), self.name)
    }

    /// Copy a referenced asset to the output directory.
    ///
    /// Returns `Ok(b)`, where `b` is whether the asset existed and was copied, `Err(_)` for a copying error.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate percent_encoding;
    /// # extern crate bloguen;
    /// # use percent_encoding::percent_decode;
    /// # use bloguen::util::LANGUAGE_EN_GB;
    /// # use bloguen::ops::BloguePost;
    /// # use std::io::{Write, Read};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use bloguen::util;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-post-copy_asset");
    /// # let _ = fs::remove_dir_all(&root);
    /// # fs::create_dir_all(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")).unwrap();
    /// # fs::create_dir_all(root.join("out").join("assets")).unwrap();
    /// # File::create(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")
    /// #                  .join("post.md")).unwrap().write_all("![img](assets/img.png)".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    /// let out_pair = ("$ROOT/out/".to_string(), root.join("out"));
    /// # File::create(out_pair.1.join("assets").join("img.png")).unwrap();
    /// let post =
    ///     BloguePost::new(("$ROOT/src/01. 2018-01-08 16-52 The venture into crocheting".to_string(),
    ///         root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting"))).unwrap();
    /// for link in post.generate(&out_pair, None, None, None, "header", "footer", "Блогг",
    ///                           &LANGUAGE_EN_GB, "autheur", &[], &[],
    ///                           &Default::default(), &Default::default(), &[], &[], &[], &[])
    ///             .unwrap().into_iter().filter(|l| util::is_asset_link(l)) {
    ///     let link = percent_decode(link.as_bytes()).decode_utf8().unwrap();
    ///     println!("Copying {}: {:?}", link, post.copy_asset(&out_pair, None, &link));
    /// }
    /// ```
    pub fn copy_asset(&self, into: &(String, PathBuf), asset_override: Option<&str>, link: &str) -> Result<bool, Error> {
        let source = concat_path(self.source_dir.1.clone(), link);
        if source.exists() {
            let output = concat_path(if let Some(ass_dir) = asset_override {
                                         concat_path(&into.1, ass_dir)
                                     } else {
                                         into.1.join("posts")
                                     },
                                     link);

            fs::create_dir_all(output.parent().unwrap()).map_err(|e| {
                    Error::Io {
                        desc: "asset parent dir".into(),
                        op: "create",
                        more: e.to_string().into(),
                    }
                })?;
            fs::copy(source, output).map_err(|e| {
                    Error::Io {
                        desc: "asset".into(),
                        op: "copy",
                        more: e.to_string().into(),
                    }
                })?;

            Ok(true)
        } else {
            Ok(false)
        }
    }
}
