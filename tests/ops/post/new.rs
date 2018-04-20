use chrono::offset::Local as LocalOffset;
use bloguen::ops::BloguePost;
use std::env::temp_dir;
use chrono::TimeZone;
use bloguen::Error;
use std::fs;


#[test]
fn ok() {
    let root = temp_dir().join("bloguen-test").join("ops-post-new-ok");
    let _ = fs::remove_dir_all(&root);
    for d in &["001. 2018-01-08 16-52 My first venture into crocheting, and what I've learned",
               "003. 2018-02-05 release-front - a generic release front-end, like Patchwork's",
               "005. 2018-04-19 23-19-21 cursed device chain"] {
        fs::create_dir_all(root.join("posts").join(d)).unwrap();
    }

    let dir = ("$ROOT/posts/001. 2018-01-08 16-52 My first venture into crocheting, and what I've learned".to_string(),
               root.join("posts").join("001. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    assert_eq!(BloguePost::new(dir.clone()),
               Ok(BloguePost {
                   source_dir: dir,
                   number: 1,
                   name: "My first venture into crocheting, and what I've learned".to_string(),
                   datetime: LocalOffset.ymd(2018, 01, 08).and_hms(16, 52, 00),
               }));

    let dir = ("$ROOT/posts/003. 2018-02-05 release-front - a generic release front-end, like Patchwork's".to_string(),
               root.join("posts").join("003. 2018-02-05 release-front - a generic release front-end, like Patchwork's"));
    assert_eq!(BloguePost::new(dir.clone()),
               Ok(BloguePost {
                   source_dir: dir,
                   number: 3,
                   name: "release-front - a generic release front-end, like Patchwork's".to_string(),
                   datetime: LocalOffset.ymd(2018, 02, 05).and_hms(12, 33, 05),
               }));

    let dir = ("$ROOT/posts/005. 2018-04-19 23-19-21 cursed device chain".to_string(), root.join("posts").join("005. 2018-04-19 23-19-21 cursed device chain"));
    assert_eq!(BloguePost::new(dir.clone()),
               Ok(BloguePost {
                   source_dir: dir,
                   number: 5,
                   name: "cursed device chain".to_string(),
                   datetime: LocalOffset.ymd(2018, 04, 19).and_hms(23, 19, 21),
               }));
}

#[test]
fn invalid_name() {
    let root = temp_dir().join("bloguen-test").join("ops-post-new-invalid_name");
    let _ = fs::remove_dir_all(&root);
    for d in &["004. stir plate"] {
        fs::create_dir_all(root.join("posts").join(d)).unwrap();
    }

    let dir = ("$ROOT/posts/004. stir plate".to_string(), root.join("posts").join("004. stir plate"));
    assert_eq!(BloguePost::new(dir.clone()),
               Err(Error::Parse {
                   tp: "post directory filename",
                   wher: "blogue post",
                   more: None,
               }));
}

#[test]
fn invalid_post_number() {
    let root = temp_dir().join("bloguen-test").join("ops-post-new-invalid_post_number");
    let _ = fs::remove_dir_all(&root);
    for d in &["99999999999999999999999999999. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"] {
        fs::create_dir_all(root.join("posts").join(d)).unwrap();
    }

    let dir = ("$ROOT/posts/99999999999999999999999999999. 2018-01-08 16-52 My first venture into crocheting, and what I've learned".to_string(),
               root.join("posts").join("99999999999999999999999999999. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    assert_eq!(BloguePost::new(dir.clone()),
               Err(Error::Parse {
                   tp: "unsigned int",
                   wher: "post number",
                   more: None,
               }));
}
