use bloguen::ops::BloguePost;
use std::env::temp_dir;
use std::fs;

mod copy_asset;
mod generate;
mod list;
mod new;


#[test]
fn normalised_name() {
    let root = temp_dir().join("bloguen-test").join("ops-post-normalised_name");
    let _ = fs::remove_dir_all(&root);
    for d in &["1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned",
               "03. 2018-02-05 release-front - a generic release front-end, like Patchwork's",
               "005. 2018-04-19 23-19-21 cursed device chain"] {
        fs::create_dir_all(root.join("posts").join(d)).unwrap();
    }

    let dir = ("$ROOT/posts/1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned".to_string(),
               root.join("posts").join("1. 2018-01-08 16-52 My first venture into crocheting, and what I've learned"));
    let post = BloguePost::new(dir.clone()).unwrap();
    assert_eq!(post.normalised_name(),
               "1. 2018-01-08 16-52-00 My first venture into crocheting, and what I've learned");

    let dir = ("$ROOT/posts/03. 2018-02-05 release-front - a generic release front-end, like Patchwork's".to_string(),
               root.join("posts").join("03. 2018-02-05 release-front - a generic release front-end, like Patchwork's"));
    let post = BloguePost::new(dir.clone()).unwrap();
    assert_eq!(post.normalised_name(),
               "03. 2018-02-05 12-33-05 release-front - a generic release front-end, like Patchwork's");

    let dir = ("$ROOT/posts/005. 2018-04-19 23-19-21 cursed device chain".to_string(), root.join("posts").join("005. 2018-04-19 23-19-21 cursed device chain"));
    let post = BloguePost::new(dir.clone()).unwrap();
    assert_eq!(post.normalised_name(), "005. 2018-04-19 23-19-21 cursed device chain");
}
