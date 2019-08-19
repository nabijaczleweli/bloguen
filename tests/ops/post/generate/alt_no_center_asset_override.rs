use bloguen::util::LANGUAGE_EN_GB;
use bloguen::ops::BloguePost;
use std::io::{Write, Read};
use std::fs::{self, File};
use std::env::temp_dir;
use bloguen::Error;
use std::str;


#[test]
fn ok() {
    let root = temp_dir().join("bloguen-test").join("ops-post-generate-alt_no_center_asset_override-ok");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned",
               "03. 2018-02-05 release-front - a generic release front-end, like Patchwork's",
               "005. 2018-04-19 23-19-21 cursed device chain"] {
        let fp = root.join("posts").join(d);
        fs::create_dir_all(&fp).unwrap();
        File::create(fp.join("post.md")).unwrap().write_all(format!("[lonk]({})", d.replace(" ", "%20")).as_bytes()).unwrap();
    }

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned/".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    let post = BloguePost::new(dir.clone()).unwrap();
    let mut alt_buf = vec![];
    assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out")),
                             Some(&mut alt_buf as &mut dyn Write),
                             None,
                             Some("overriden-assets"),
                             "header",
                             "footer",
                             "Блогг",
                             &LANGUAGE_EN_GB,
                             "autheur",
                             &[],
                             &[],
                             &Default::default(),
                             &Default::default(),
                             &[],
                             &[],
                             &[],
                             &[]),
               Ok(vec!["1.%202018-01-08%2016-52%20My%20first%20venture%20into%20crocheting,%20and%20what%20I've%20learned".to_string()]));
    let mut read = String::new();
    File::open(root.join("out").join("posts").join(post.normalised_name() + ".html")).unwrap().read_to_string(&mut read).unwrap();
    assert_eq!(read,
               "header<p><a href=\"1.%202018-01-08%2016-52%20My%20first%20venture%20into%20crocheting,%20and%20what%20I've%20learned\">lonk</a></p>\nfooter");
    assert_eq!(str::from_utf8(&alt_buf).unwrap(),
               "<p><a href=\"1.%202018-01-08%2016-52%20My%20first%20venture%20into%20crocheting,%20and%20what%20I've%20learned\">lonk</a></p>\n");

    let dir = ("$ROOT/posts/03. 2018-02-05 release-front - a generic release front-end, like Patchwork's/".to_string(),
               root.join("posts").join("03. 2018-02-05 release-front - a generic release front-end, like Patchwork's"));
    let post = BloguePost::new(dir.clone()).unwrap();
    alt_buf.clear();
    assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out")),
                             Some(&mut alt_buf as &mut dyn Write),
                             None,
                             Some("overriden-assets"),
                             "header",
                             "footer",
                             "Блогг",
                             &LANGUAGE_EN_GB,
                             "autheur",
                             &[],
                             &[],
                             &Default::default(),
                             &Default::default(),
                             &[],
                             &[],
                             &[],
                             &[]),
               Ok(vec!["03.%202018-02-05%20release-front%20-%20a%20generic%20release%20front-end,%20like%20Patchwork's".to_string()]));
    read.clear();
    File::open(root.join("out").join("posts").join(post.normalised_name() + ".html")).unwrap().read_to_string(&mut read).unwrap();
    assert_eq!(read,
               "header<p><a href=\"03.%202018-02-05%20release-front%20-%20a%20generic%20release%20front-end,%20like%20Patchwork's\">lonk</a></p>\nfooter");
    assert_eq!(str::from_utf8(&alt_buf).unwrap(),
               "<p><a href=\"03.%202018-02-05%20release-front%20-%20a%20generic%20release%20front-end,%20like%20Patchwork's\">lonk</a></p>\n");

    let dir = ("$ROOT/posts/005. 2018-04-19 23-19-21 cursed device chain/".to_string(),
               root.join("posts").join("005. 2018-04-19 23-19-21 cursed device chain"));
    let post = BloguePost::new(dir.clone()).unwrap();
    alt_buf.clear();
    assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out")),
                             Some(&mut alt_buf as &mut dyn Write),
                             None,
                             Some("overriden-assets"),
                             "header",
                             "footer",
                             "Блогг",
                             &LANGUAGE_EN_GB,
                             "autheur",
                             &[],
                             &[],
                             &Default::default(),
                             &Default::default(),
                             &[],
                             &[],
                             &[],
                             &[]),
               Ok(vec!["005.%202018-04-19%2023-19-21%20cursed%20device%20chain".to_string()]));
    read.clear();
    File::open(root.join("out").join("posts").join(post.normalised_name() + ".html")).unwrap().read_to_string(&mut read).unwrap();
    assert_eq!(read,
               "header<p><a href=\"005.%202018-04-19%2023-19-21%20cursed%20device%20chain\">lonk</a></p>\nfooter");
    assert_eq!(str::from_utf8(&alt_buf).unwrap(),
               "<p><a href=\"005.%202018-04-19%2023-19-21%20cursed%20device%20chain\">lonk</a></p>\n");
}

#[test]
fn not_found() {
    let root = temp_dir().join("bloguen-test").join("ops-post-generate-alt_no_center_asset_override-not_found");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"] {
        fs::create_dir_all(root.join("posts").join(d)).unwrap();
    }

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned/".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    let post = BloguePost::new(dir.clone()).unwrap();
    let mut alt_buf = vec![];
    assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out")),
                             Some(&mut alt_buf as &mut dyn Write),
                             None,
                             Some("overriden-assets"),
                             "header",
                             "footer",
                             "Блогг",
                             &LANGUAGE_EN_GB,
                             "autheur",
                             &[],
                             &[],
                             &Default::default(),
                             &Default::default(),
                             &[],
                             &[],
                             &[],
                             &[]),
               Err(Error::FileNotFound {
                   who: "post text",
                   path: format!("{}post.md", dir.0).into(),
               }));
    assert!(alt_buf.is_empty());
}

#[test]
fn non_utf8() {
    let root = temp_dir().join("bloguen-test").join("ops-post-generate-alt_no_center_asset_override-not_utf8");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"] {
        let fp = root.join("posts").join(d);
        fs::create_dir_all(&fp).unwrap();
        File::create(fp.join("post.md"))
            .unwrap()
            .write_all(&[0xC3, 0x28, 0xA0, 0xA1, 0xE2, 0x28, 0xA1, 0xE2, 0x82, 0x28, 0xF0, 0x28, 0x8C, 0xBC, 0xF0, 0x90, 0x28, 0xBC, 0xF0, 0x28, 0x8C, 0x28])
            .unwrap();
    }

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned/".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    let post = BloguePost::new(dir.clone()).unwrap();
    let mut alt_buf = vec![];
    assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out")),
                             Some(&mut alt_buf as &mut dyn Write),
                             None,
                             Some("overriden-assets"),
                             "header",
                             "footer",
                             "Блогг",
                             &LANGUAGE_EN_GB,
                             "autheur",
                             &[],
                             &[],
                             &Default::default(),
                             &Default::default(),
                             &[],
                             &[],
                             &[],
                             &[]),
               Err(Error::Parse {
                   tp: "UTF-8 string",
                   wher: "post text".into(),
                   more: "stream did not contain valid UTF-8".into(),
               }));
    assert!(alt_buf.is_empty());
}

#[test]
fn posts_directory() {
    let root = temp_dir().join("bloguen-test").join("ops-post-generate-alt_no_center_asset_override-posts_directory");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"] {
        let fp = root.join("posts").join(d);
        fs::create_dir_all(&fp).unwrap();
        File::create(fp.join("post.md")).unwrap().write_all(d.as_bytes()).unwrap();
    }

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned/".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    let post = BloguePost::new(dir.clone()).unwrap();
    fs::create_dir_all(root.join("out")).unwrap();
    File::create(root.join("out").join("posts")).unwrap().write_all("henlo".as_bytes()).unwrap();
    let mut alt_buf = vec![];
    assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out")),
                             Some(&mut alt_buf as &mut dyn Write),
                             None,
                             Some("overriden-assets"),
                             "header",
                             "footer",
                             "Блогг",
                             &LANGUAGE_EN_GB,
                             "autheur",
                             &[],
                             &[],
                             &Default::default(),
                             &Default::default(),
                             &[],
                             &[],
                             &[],
                             &[]),
               Err(Error::Io {
                   desc: "posts directory".into(),
                   op: "create",
                   more: if cfg!(target_os = "windows") {
                           "Cannot create a file when that file already exists. (os error 183)"
                       } else {
                           "File exists (os error 17)"
                       }
                       .into(),
               }));
    assert!(alt_buf.is_empty());
}

#[test]
fn post_create() {
    let root = temp_dir().join("bloguen-test").join("ops-post-generate-alt_no_center_asset_override-post_create");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"] {
        let fp = root.join("posts").join(d);
        fs::create_dir_all(&fp).unwrap();
        File::create(fp.join("post.md")).unwrap().write_all(d.as_bytes()).unwrap();
    }

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned/".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    let post = BloguePost::new(dir.clone()).unwrap();
    fs::create_dir_all(root.join("out").join("posts").join("1. 2018-01-08 16-52-00 My first venture into crocheting, and what I've learned.html")).unwrap();
    let mut alt_buf = vec![];
    assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out")),
                             Some(&mut alt_buf as &mut dyn Write),
                             None,
                             Some("overriden-assets"),
                             "header",
                             "footer",
                             "Блогг",
                             &LANGUAGE_EN_GB,
                             "autheur",
                             &[],
                             &[],
                             &Default::default(),
                             &Default::default(),
                             &[],
                             &[],
                             &[],
                             &[]),
               Err(Error::Io {
                   desc: "post HTML".into(),
                   op: "create",
                   more: if cfg!(target_os = "windows") {
                           "Access is denied. (os error 5)"
                       } else {
                           "Is a directory (os error 21)"
                       }
                       .into(),
               }));
    assert!(alt_buf.is_empty());
}
