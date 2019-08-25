use percent_encoding::percent_decode;
use bloguen::util::LANGUAGE_EN_GB;
use bloguen::ops::BloguePost;
use std::io::{Write, Read};
use std::fs::{self, File};
use std::env::temp_dir;
use bloguen::Error;


#[test]
fn ok_copied() {
    let root = temp_dir().join("bloguen-test").join("ops-post-copy_asset-no_asset_override-ok_copied");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 Big speakers",
               "03. 2018-02-05 release-front - a generic release front-end like Patchworks",
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

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 Big speakers".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 Big speakers"));
    let post = BloguePost::new(dir.clone()).unwrap();
    for link in post.generate(&out_dir,
                  None,
                  None,
                  None,
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
        assert_eq!(post.copy_asset(&out_dir, None, &percent_decode(link.as_bytes()).decode_utf8().unwrap()),
                   Ok(true));
    }
    let mut read = String::new();
    File::open(out_dir.1.join("posts").join("1. 2018-01-08 16-52 Big speakers.bin"))
        .unwrap()
        .read_to_string(&mut read)
        .unwrap();
    assert_eq!(read, "1. 2018-01-08 16-52 Big speakers");
    read.clear();
    File::open(out_dir.1.join("posts").join("assets").join("image.png")).unwrap().read_to_string(&mut read).unwrap();
    assert_eq!(read, "1. 2018-01-08 16-52 Big speakers");

    let dir = ("$ROOT/posts/03. 2018-02-05 release-front - a generic release front-end like Patchworks".to_string(),
               root.join("posts").join("03. 2018-02-05 release-front - a generic release front-end like Patchworks"));
    let post = BloguePost::new(dir.clone()).unwrap();
    for link in post.generate(&out_dir,
                  None,
                  None,
                  None,
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
        assert_eq!(post.copy_asset(&out_dir, None, &percent_decode(link.as_bytes()).decode_utf8().unwrap()),
                   Ok(true));
    }
    read.clear();
    File::open(out_dir.1.join("posts").join("03. 2018-02-05 release-front - a generic release front-end like Patchworks.bin"))
        .unwrap()
        .read_to_string(&mut read)
        .unwrap();
    assert_eq!(read, "03. 2018-02-05 release-front - a generic release front-end like Patchworks");
    read.clear();
    File::open(out_dir.1.join("posts").join("assets").join("image.png")).unwrap().read_to_string(&mut read).unwrap();
    assert_eq!(read, "03. 2018-02-05 release-front - a generic release front-end like Patchworks");

    let dir = ("$ROOT/posts/005. 2018-04-19 23-19-21 cursed device chain".to_string(), root.join("posts").join("005. 2018-04-19 23-19-21 cursed device chain"));
    let post = BloguePost::new(dir.clone()).unwrap();
    for link in post.generate(&out_dir,
                  None,
                  None,
                  None,
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
        assert_eq!(post.copy_asset(&out_dir, None, &percent_decode(link.as_bytes()).decode_utf8().unwrap()),
                   Ok(true));
    }
    read.clear();
    File::open(out_dir.1.join("posts").join("005. 2018-04-19 23-19-21 cursed device chain.bin")).unwrap().read_to_string(&mut read).unwrap();
    assert_eq!(read, "005. 2018-04-19 23-19-21 cursed device chain");
    read.clear();
    File::open(out_dir.1.join("posts").join("assets").join("image.png")).unwrap().read_to_string(&mut read).unwrap();
    assert_eq!(read, "005. 2018-04-19 23-19-21 cursed device chain");
}

#[test]
fn ok_not_copied() {
    let root = temp_dir().join("bloguen-test").join("ops-post-copy_asset-no_asset_override-ok_not_copied");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 Big speakers",
               "03. 2018-02-05 release-front - a generic release front-end like Patchworks",
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

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 Big speakers".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 Big speakers"));
    let post = BloguePost::new(dir.clone()).unwrap();
    for link in post.generate(&out_dir,
                  None,
                  None,
                  None,
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
        assert_eq!(post.copy_asset(&out_dir, None, &percent_decode(link.as_bytes()).decode_utf8().unwrap()),
                   Ok(false));
    }
    assert!(File::open(out_dir.1.join("posts").join("1. 2018-01-08 16-52 Big speakers.bin")).is_err());
    assert!(File::open(out_dir.1.join("posts").join("assets").join("image.png")).is_err());

    let dir = ("$ROOT/posts/03. 2018-02-05 release-front - a generic release front-end like Patchworks".to_string(),
               root.join("posts").join("03. 2018-02-05 release-front - a generic release front-end like Patchworks"));
    let post = BloguePost::new(dir.clone()).unwrap();
    for link in post.generate(&out_dir,
                  None,
                  None,
                  None,
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
        assert_eq!(post.copy_asset(&out_dir, None, &percent_decode(link.as_bytes()).decode_utf8().unwrap()),
                   Ok(false));
    }
    assert!(File::open(out_dir.1.join("posts").join("03. 2018-02-05 release-front - a generic release front-end like Patchworks.bin")).is_err());
    assert!(File::open(out_dir.1.join("posts").join("assets").join("image.png")).is_err());

    let dir = ("$ROOT/posts/005. 2018-04-19 23-19-21 cursed device chain".to_string(), root.join("posts").join("005. 2018-04-19 23-19-21 cursed device chain"));
    let post = BloguePost::new(dir.clone()).unwrap();
    for link in post.generate(&out_dir,
                  None,
                  None,
                  None,
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
        assert_eq!(post.copy_asset(&out_dir, None, &percent_decode(link.as_bytes()).decode_utf8().unwrap()),
                   Ok(false));
    }
    assert!(File::open(out_dir.1.join("posts").join("005. 2018-04-19 23-19-21 cursed device chain.bin")).is_err());
    assert!(File::open(out_dir.1.join("posts").join("assets").join("image.png")).is_err());
}

#[test]
fn posts_directory() {
    let root = temp_dir().join("bloguen-test").join("ops-post-copy_asset-no_asset_override-posts_directory");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 Big speakers"] {
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

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 Big speakers".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 Big speakers"));
    let post = BloguePost::new(dir.clone()).unwrap();
    fs::create_dir_all(&out_dir.1).unwrap();
    File::create(out_dir.1.join("posts")).unwrap();
    assert_eq!(post.copy_asset(&out_dir, None, "assets/image.png"),
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
                               None,
                               "1. 2018-01-08 16-52 Big speakers.bin"),
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
    let root = temp_dir().join("bloguen-test").join("ops-post-copy_asset-no_asset_override-copy_failed");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 Big speakers"] {
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

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 Big speakers".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 Big speakers"));
    let post = BloguePost::new(dir.clone()).unwrap();
    fs::create_dir_all(out_dir.1.join("posts").join("assets").join("image.png")).unwrap();
    fs::create_dir_all(out_dir.1.join("posts").join("1. 2018-01-08 16-52 Big speakers.bin")).unwrap();
    for link in post.generate(&out_dir,
                  None,
                  None,
                  None,
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
        assert_eq!(post.copy_asset(&out_dir, None, &percent_decode(link.as_bytes()).decode_utf8().unwrap()),
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
