use self::super::super::util::{MARKDOWN_OPTIONS, name_based_post_time};
use walkdir::{Error as WalkDirError, DirEntry, WalkDir};
use chrono::{NaiveTime, DateTime, TimeZone};
use chrono::offset::Local as LocalOffset;
use comrak::{self, Arena as ComrakArena};
use self::super::super::Error;
use std::iter::FromIterator;
use std::fs::{self, File};
use std::path::PathBuf;
use std::io::Read;
use regex::Regex;


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
                    desc: "post list",
                    op: "list",
                    more: Some(e.to_string()),
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
    ///                wher: "blogue post",
    ///                more: None,
    ///            }));
    /// ```
    pub fn new(wher: (String, PathBuf)) -> Result<BloguePost, Error> {
        fn uint_err(wher: &'static str) -> Error {
            Error::Parse {
                tp: "unsigned int",
                wher: wher,
                more: None,
            }
        }


        let mut ret = {
            let mch = POST_DIR_NAME.captures(wher.1.file_name().unwrap().to_str().unwrap())
                .ok_or_else(|| {
                    Error::Parse {
                        tp: "post directory filename",
                        wher: "blogue post",
                        more: None,
                    }
                })?;
            let name = mch.name("name").unwrap().as_str();
            let number = mch.name("post_number").unwrap().as_str();

            BloguePost {
                source_dir: (String::new(), PathBuf::from(String::new())),
                number: (number.parse().map_err(|_| uint_err("post number"))?, number.to_string()),
                name: name.to_string(),
                datetime: LocalOffset.ymd(mch.name("date_year").unwrap().as_str().parse().map_err(|_| uint_err("post date year"))?,
                         mch.name("date_month").unwrap().as_str().parse().map_err(|_| uint_err("post date month"))?,
                         mch.name("date_day").unwrap().as_str().parse().map_err(|_| uint_err("post date day"))?)
                    .and_time(if let Some(hour) = mch.name("time_hour") {
                        NaiveTime::from_hms(hour.as_str().parse().map_err(|_| uint_err("post time hour"))?,
                                            mch.name("time_minute").unwrap().as_str().parse().map_err(|_| uint_err("post time minute"))?,
                                            if let Some(second) = mch.name("time_second") {
                                                second.as_str().parse().map_err(|_| uint_err("post time second"))?
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
    /// # use bloguen::ops::BloguePost;
    /// # use std::io::{Write, Read};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-post-generate");
    /// # let _ = fs::remove_dir_all(&root);
    /// # fs::create_dir_all(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")).unwrap();
    /// # File::create(root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting")
    /// #                  .join("post.md")).unwrap().write_all("Блогг".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    /// let post =
    ///     BloguePost::new(("$ROOT/src/01. 2018-01-08 16-52 The venture into crocheting".to_string(),
    ///                     root.join("src").join("01. 2018-01-08 16-52 The venture into crocheting"))).unwrap();
    /// assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out"))), Ok(()));
    ///
    /// assert!(root.join("out").join("posts").join("01. 2018-01-08 16-52-00 The venture into crocheting.html").is_file());
    /// # let mut read = vec![];
    /// # File::open(root.join("out").join("posts").join("01. 2018-01-08 16-52-00 The venture into crocheting.html")).unwrap().read_to_end(&mut read).unwrap();
    /// # assert_eq!(&read[..], "<p>Блогг</p>\n".as_bytes());
    /// ```
    pub fn generate(&self, into: &(String, PathBuf)) -> Result<(), Error> {
        let post_text_path = self.source_dir.1.join("post.md");
        let mut post_text = String::new();
        File::open(&post_text_path).map_err(|_| {
                Error::FileNotFound {
                    who: "post text",
                    path: post_text_path,
                }
            })?
            .read_to_string(&mut post_text)
            .map_err(|e| {
                Error::Io {
                    desc: "post text",
                    op: "read",
                    more: Some(e.to_string()),
                }
            })?;

        let arena = ComrakArena::new();
        let root = comrak::parse_document(&arena, &post_text, &MARKDOWN_OPTIONS);

        fs::create_dir_all(into.1.join("posts")).map_err(|e| {
                Error::Io {
                    desc: "posts directory",
                    op: "create",
                    more: Some(e.to_string()),
                }
            })?;

        // TODO extract arg of second join
        let post_html_path = into.1.join("posts").join(format!("{}. {} {}.html", self.number.1, self.datetime.format("%Y-%m-%d %H-%M-%S"), self.name));
        comrak::format_html(root,
                            &MARKDOWN_OPTIONS,
                            &mut File::create(post_html_path).map_err(|e| {
                Error::Io {
                    desc: "post HTML",
                    op: "create",
                    more: Some(e.to_string()),
                }
            })?).map_err(|e| {
                Error::Io {
                    desc: "post HTML",
                    op: "write",
                    more: Some(e.to_string()),
                }
            })?;

        Ok(())
    }
}
