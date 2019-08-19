use percent_encoding::percent_decode;
use bloguen::util::LANGUAGE_EN_GB;
use bloguen::ops::BloguePost;
use std::io::{Write, Read};
use std::fs::{self, File};
use std::env::temp_dir;
use bloguen::Error;


#[test]
fn ok_copied() {
    let root = temp_dir().join("bloguen-test").join("ops-post-copy_asset-asset_override-ok_copied");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned",
               "03. 2018-02-05 release-front - a generic release front-end, like Patchwork's",
               "005. 2018-04-19 23-19-21 cursed device chain"] {
        let fp = root.join("posts").join(d);
        fs::create_dir_all(fp.join("assets")).unwrap();
        File::create(fp.join("post.md"))
            .unwrap()
            .write_all(format!("[self]({}.bin)\n\
                                ![img](assets/image.png)",
                               d.replace(" ", "%20"))
                .as_bytes())
            .unwrap();
        File::create(fp.join(format!("{}.bin", d))).unwrap().write_all(d.as_bytes()).unwrap();
        File::create(fp.join("assets").join("image.png")).unwrap().write_all(d.as_bytes()).unwrap();
    }
    let out_dir = ("$ROOT/out/".to_string(), root.join("out"));

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    let post = BloguePost::new(dir.clone()).unwrap();
    for link in post.generate(&out_dir,
                  None,
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
                  &[])
        .unwrap() {
        assert_eq!(post.copy_asset(&out_dir, Some("overriden-assets"), &percent_decode(link.as_bytes()).decode_utf8().unwrap()),
                   Ok(true));
    }
    let mut read = String::new();
    File::open(out_dir.1.join("overriden-assets").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned.bin"))
        .unwrap()
        .read_to_string(&mut read)
        .unwrap();
    assert_eq!(read, "1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned");
    read.clear();
    File::open(out_dir.1.join("overriden-assets").join("assets").join("image.png")).unwrap().read_to_string(&mut read).unwrap();
    assert_eq!(read, "1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned");

    let dir = ("$ROOT/posts/03. 2018-02-05 release-front - a generic release front-end, like Patchwork's".to_string(),
               root.join("posts").join("03. 2018-02-05 release-front - a generic release front-end, like Patchwork's"));
    let post = BloguePost::new(dir.clone()).unwrap();
    for link in post.generate(&out_dir,
                  None,
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
                  &[])
        .unwrap() {
        assert_eq!(post.copy_asset(&out_dir, Some("overriden-assets"), &percent_decode(link.as_bytes()).decode_utf8().unwrap()),
                   Ok(true));
    }
    read.clear();
    File::open(out_dir.1.join("overriden-assets").join("03. 2018-02-05 release-front - a generic release front-end, like Patchwork's.bin"))
        .unwrap()
        .read_to_string(&mut read)
        .unwrap();
    assert_eq!(read, "03. 2018-02-05 release-front - a generic release front-end, like Patchwork's");
    read.clear();
    File::open(out_dir.1.join("overriden-assets").join("assets").join("image.png")).unwrap().read_to_string(&mut read).unwrap();
    assert_eq!(read, "03. 2018-02-05 release-front - a generic release front-end, like Patchwork's");

    let dir = ("$ROOT/posts/005. 2018-04-19 23-19-21 cursed device chain".to_string(), root.join("posts").join("005. 2018-04-19 23-19-21 cursed device chain"));
    let post = BloguePost::new(dir.clone()).unwrap();
    for link in post.generate(&out_dir,
                  None,
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
                  &[])
        .unwrap() {
        assert_eq!(post.copy_asset(&out_dir, Some("overriden-assets"), &percent_decode(link.as_bytes()).decode_utf8().unwrap()),
                   Ok(true));
    }
    read.clear();
    File::open(out_dir.1.join("overriden-assets").join("005. 2018-04-19 23-19-21 cursed device chain.bin")).unwrap().read_to_string(&mut read).unwrap();
    assert_eq!(read, "005. 2018-04-19 23-19-21 cursed device chain");
    read.clear();
    File::open(out_dir.1.join("overriden-assets").join("assets").join("image.png")).unwrap().read_to_string(&mut read).unwrap();
    assert_eq!(read, "005. 2018-04-19 23-19-21 cursed device chain");
}

#[test]
fn ok_not_copied() {
    let root = temp_dir().join("bloguen-test").join("ops-post-copy_asset-asset_override-ok_not_copied");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned",
               "03. 2018-02-05 release-front - a generic release front-end, like Patchwork's",
               "005. 2018-04-19 23-19-21 cursed device chain"] {
        let fp = root.join("posts").join(d);
        fs::create_dir_all(fp.join("assets")).unwrap();
        File::create(fp.join("post.md"))
            .unwrap()
            .write_all(format!("[self]({}.bin)\n\
                                ![img](assets/image.png)",
                               d.replace(" ", "%20"))
                .as_bytes())
            .unwrap();
    }
    let out_dir = ("$ROOT/out/".to_string(), root.join("out"));

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    let post = BloguePost::new(dir.clone()).unwrap();
    for link in post.generate(&out_dir,
                  None,
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
                  &[])
        .unwrap() {
        assert_eq!(post.copy_asset(&out_dir, Some("overriden-assets"), &percent_decode(link.as_bytes()).decode_utf8().unwrap()),
                   Ok(false));
    }
    assert!(File::open(out_dir.1.join("overriden-assets").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned.bin")).is_err());
    assert!(File::open(out_dir.1.join("overriden-assets").join("assets").join("image.png")).is_err());

    let dir = ("$ROOT/posts/03. 2018-02-05 release-front - a generic release front-end, like Patchwork's".to_string(),
               root.join("posts").join("03. 2018-02-05 release-front - a generic release front-end, like Patchwork's"));
    let post = BloguePost::new(dir.clone()).unwrap();
    for link in post.generate(&out_dir,
                  None,
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
                  &[])
        .unwrap() {
        assert_eq!(post.copy_asset(&out_dir, Some("overriden-assets"), &percent_decode(link.as_bytes()).decode_utf8().unwrap()),
                   Ok(false));
    }
    assert!(File::open(out_dir.1.join("overriden-assets").join("03. 2018-02-05 release-front - a generic release front-end, like Patchwork's.bin")).is_err());
    assert!(File::open(out_dir.1.join("overriden-assets").join("assets").join("image.png")).is_err());

    let dir = ("$ROOT/posts/005. 2018-04-19 23-19-21 cursed device chain".to_string(), root.join("posts").join("005. 2018-04-19 23-19-21 cursed device chain"));
    let post = BloguePost::new(dir.clone()).unwrap();
    for link in post.generate(&out_dir,
                  None,
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
                  &[])
        .unwrap() {
        assert_eq!(post.copy_asset(&out_dir, Some("overriden-assets"), &percent_decode(link.as_bytes()).decode_utf8().unwrap()),
                   Ok(false));
    }
    assert!(File::open(out_dir.1.join("overriden-assets").join("005. 2018-04-19 23-19-21 cursed device chain.bin")).is_err());
    assert!(File::open(out_dir.1.join("overriden-assets").join("assets").join("image.png")).is_err());
}

#[test]
fn posts_directory() {
    let root = temp_dir().join("bloguen-test").join("ops-post-copy_asset-asset_override-posts_directory");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"] {
        let fp = root.join("posts").join(d);
        fs::create_dir_all(fp.join("assets")).unwrap();
        File::create(fp.join("post.md"))
            .unwrap()
            .write_all(format!("[self]({}.bin)\n\
                                ![img](assets/image.png)",
                               d.replace(" ", "%20"))
                .as_bytes())
            .unwrap();
        File::create(fp.join(format!("{}.bin", d))).unwrap().write_all(d.as_bytes()).unwrap();
        File::create(fp.join("assets").join("image.png")).unwrap().write_all(d.as_bytes()).unwrap();
    }
    let out_dir = ("$ROOT/out/".to_string(), root.join("out"));

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    let post = BloguePost::new(dir.clone()).unwrap();
    fs::create_dir_all(&out_dir.1).unwrap();
    File::create(out_dir.1.join("overriden-assets")).unwrap();
    assert_eq!(post.copy_asset(&out_dir, Some("overriden-assets"), "assets/image.png"),
               Err(Error::Io {
                   desc: "asset parent dir".into(),
                   op: "create",
                   more: if cfg!(target_os = "windows") {
                           "Cannot create a file when that file already exists. (os error 183)"
                       } else {
                           "Not a directory (os error 20)"
                       }
                       .into(),
               }));
    assert_eq!(post.copy_asset(&out_dir,
                               Some("overriden-assets"),
                               "1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned.bin"),
               Err(Error::Io {
                   desc: "asset parent dir".into(),
                   op: "create",
                   more: if cfg!(target_os = "windows") {
                           "Cannot create a file when that file already exists. (os error 183)"
                       } else {
                           "File exists (os error 17)"
                       }
                       .into(),
               }));
}

#[test]
fn copy_failed() {
    let root = temp_dir().join("bloguen-test").join("ops-post-copy_asset-asset_override-copy_failed");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"] {
        let fp = root.join("posts").join(d);
        fs::create_dir_all(fp.join("assets")).unwrap();
        File::create(fp.join("post.md"))
            .unwrap()
            .write_all(format!("[self]({}.bin)\n\
                                ![img](assets/image.png)",
                               d.replace(" ", "%20"))
                .as_bytes())
            .unwrap();
        File::create(fp.join(format!("{}.bin", d))).unwrap().write_all(d.as_bytes()).unwrap();
        File::create(fp.join("assets").join("image.png")).unwrap().write_all(d.as_bytes()).unwrap();
    }
    let out_dir = ("$ROOT/out/".to_string(), root.join("out"));

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    let post = BloguePost::new(dir.clone()).unwrap();
    fs::create_dir_all(out_dir.1.join("overriden-assets").join("assets").join("image.png")).unwrap();
    fs::create_dir_all(out_dir.1.join("overriden-assets").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned.bin")).unwrap();
    for link in post.generate(&out_dir,
                  None,
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
                  &[])
        .unwrap() {
        assert_eq!(post.copy_asset(&out_dir, Some("overriden-assets"), &percent_decode(link.as_bytes()).decode_utf8().unwrap()),
                   Err(Error::Io {
                       desc: "asset".into(),
                       op: "copy",
                       more: if cfg!(target_os = "windows") {
                               "Access is denied. (os error 5)"
                           } else {
                               "Is a directory (os error 21)"
                           }
                           .into(),
                   }));
    }
}
