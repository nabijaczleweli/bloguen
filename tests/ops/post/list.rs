use bloguen::ops::BloguePost;
use std::fs::{self, File};
use std::env::temp_dir;


#[test]
fn ok() {
    let root = temp_dir().join("bloguen-test").join("ops-post-list-ok");
    let _ = fs::remove_dir_all(&root);
    for d in &["temp",
               "001. 2018-01-08 16-52 My first venture into crocheting, and what I've learned",
               "003. 2018-02-05 release-front - a generic release front-end, like Patchwork's",
               "004. stir plate"] {
        fs::create_dir_all(root.join("posts").join(d)).unwrap();
    }
    for f in &["002. 2018-01-08 acquiescence.md", "blogue.toml"] {
        File::create(root.join("posts").join(f)).unwrap();
    }

    let posts = BloguePost::list(&("$ROOT/posts/".to_string(), root.join("posts"))).unwrap();
    assert_eq!(&posts[..],
               &[("$ROOT/posts/001. 2018-01-08 16-52 My first venture into crocheting, and what I've learned/".to_string(),
                  root.join("posts").join("001. 2018-01-08 16-52 My first venture into crocheting, and what I've learned")),
                 ("$ROOT/posts/003. 2018-02-05 release-front - a generic release front-end, like Patchwork's/".to_string(),
                  root.join("posts").join("003. 2018-02-05 release-front - a generic release front-end, like Patchwork's"))]
                    [..]);
}

// Not sure how to make this error programmatically
