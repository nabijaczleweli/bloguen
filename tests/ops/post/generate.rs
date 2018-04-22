use bloguen::ops::BloguePost;
use std::io::{Write, Read};
use std::fs::{self, File};
use std::env::temp_dir;
use bloguen::Error;


#[test]
fn ok() {
    let root = temp_dir().join("bloguen-test").join("ops-post-generate-ok");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned",
               "03. 2018-02-05 release-front - a generic release front-end, like Patchwork's",
               "005. 2018-04-19 23-19-21 cursed device chain"] {
        let fp = root.join("posts").join(d);
        fs::create_dir_all(&fp).unwrap();
        File::create(fp.join("post.md")).unwrap().write_all(d.as_bytes()).unwrap();
    }

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    let post = BloguePost::new(dir.clone()).unwrap();
    assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out"))), Ok(()));
    let mut read = String::new();
    File::open(root.join("out").join("posts").join(post.normalised_name() + ".html"))
        .unwrap()
        .read_to_string(&mut read)
        .unwrap();
    assert_eq!(read,
               "<ol>\n<li>2018-01-08 16-52 My first venture into crocheting, and what I've learned</li>\n</ol>\n");

    let dir = ("$ROOT/posts/03. 2018-02-05 release-front - a generic release front-end, like Patchwork's".to_string(),
               root.join("posts").join("03. 2018-02-05 release-front - a generic release front-end, like Patchwork's"));
    let post = BloguePost::new(dir.clone()).unwrap();
    assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out"))), Ok(()));
    read.clear();
    File::open(root.join("out").join("posts").join(post.normalised_name() + ".html"))
        .unwrap()
        .read_to_string(&mut read)
        .unwrap();
    assert_eq!(read,
               "<ol start=\"3\">\n<li>2018-02-05 release-front - a generic release front-end, like Patchwork's</li>\n</ol>\n");

    let dir = ("$ROOT/posts/005. 2018-04-19 23-19-21 cursed device chain".to_string(), root.join("posts").join("005. 2018-04-19 23-19-21 cursed device chain"));
    let post = BloguePost::new(dir.clone()).unwrap();
    assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out"))), Ok(()));
    read.clear();
    File::open(root.join("out").join("posts").join(post.normalised_name() + ".html")).unwrap().read_to_string(&mut read).unwrap();
    assert_eq!(read, "<ol start=\"5\">\n<li>2018-04-19 23-19-21 cursed device chain</li>\n</ol>\n");
}

#[test]
fn not_found() {
    let root = temp_dir().join("bloguen-test").join("ops-post-generate-not_found");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"] {
        fs::create_dir_all(root.join("posts").join(d)).unwrap();
    }

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    let post = BloguePost::new(dir.clone()).unwrap();
    assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out"))),
               Err(Error::FileNotFound {
                   who: "post text",
                   path: dir.1.join("post.md"),
               }));
}

#[test]
fn non_utf8() {
    let root = temp_dir().join("bloguen-test").join("ops-post-generate-not_utf8");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"] {
        let fp = root.join("posts").join(d);
        fs::create_dir_all(&fp).unwrap();
        File::create(fp.join("post.md"))
            .unwrap()
            .write_all(&[0xC3, 0x28, 0xA0, 0xA1, 0xE2, 0x28, 0xA1, 0xE2, 0x82, 0x28, 0xF0, 0x28, 0x8C, 0xBC, 0xF0, 0x90, 0x28, 0xBC, 0xF0, 0x28, 0x8C, 0x28])
            .unwrap();
    }

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    let post = BloguePost::new(dir.clone()).unwrap();
    assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out"))),
               Err(Error::Io {
                   desc: "post text",
                   op: "read",
                   more: Some("stream did not contain valid UTF-8".to_string()),
               }));
}

#[test]
fn posts_directory() {
    let root = temp_dir().join("bloguen-test").join("ops-post-generate-posts_directory");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"] {
        let fp = root.join("posts").join(d);
        fs::create_dir_all(&fp).unwrap();
        File::create(fp.join("post.md")).unwrap().write_all(d.as_bytes()).unwrap();
    }

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    let post = BloguePost::new(dir.clone()).unwrap();
    fs::create_dir_all(root.join("out")).unwrap();
    File::create(root.join("out").join("posts")).unwrap().write_all("henlo".as_bytes()).unwrap();
    assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out"))),
               Err(Error::Io {
                   desc: "posts directory",
                   op: "create",
                   more: Some("Cannot create a file when that file already exists. (os error 183)".to_string()),
               }));
}

#[test]
fn post_create() {
    let root = temp_dir().join("bloguen-test").join("ops-post-generate-post_create");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"] {
        let fp = root.join("posts").join(d);
        fs::create_dir_all(&fp).unwrap();
        File::create(fp.join("post.md")).unwrap().write_all(d.as_bytes()).unwrap();
    }

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    let post = BloguePost::new(dir.clone()).unwrap();
    fs::create_dir_all(root.join("out").join("posts").join("1. 2018-01-08 16-52-00 My first venture into crocheting, and what I've learned.html")).unwrap();
    assert_eq!(post.generate(&("$ROOT/out/".to_string(), root.join("out"))),
               Err(Error::Io {
                   desc: "post HTML",
                   op: "create",
                   more: Some("Access is denied. (os error 5)".to_string()),
               }));
}
