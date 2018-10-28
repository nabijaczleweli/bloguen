//! Not quite sure how to test the non-UTF-8 error case, since the document is parsed from a UTF-8 string


use bloguen::util::{override_assets, MARKDOWN_OPTIONS};
use comrak::{self, Arena as ComrakArena};
use std::fs::{self, File};
use std::env::temp_dir;
use std::io::Write;


#[test]
fn depth_0() {
    depth_test(0, "assets/");
}

#[test]
fn depth_1() {
    depth_test(1, "../assets/");
}

#[test]
fn depth_2() {
    depth_test(2, "../../assets/");
}


fn depth_test(depth: usize, img_prepath: &str) {
    let root = temp_dir().join("bloguen-test").join(format!("util-override_assets-depth_{}", depth));
    fs::create_dir_all(&root).unwrap();
    File::create(root.join("image.png")).unwrap().write_all("image.png".as_bytes()).unwrap();
    File::create(root.join("link.html")).unwrap().write_all("link.html".as_bytes()).unwrap();

    let doc_arena = ComrakArena::new();
    let ast = comrak::parse_document(&doc_arena,
                                     r#"[not-link](not-link.html)
                                        [link](link.html)
                                        ![img](image.png)
                                        [наб](https://nabijaczleweli.xyz)"#,
                                     &MARKDOWN_OPTIONS);
    assert_eq!(override_assets(&root, "assets/", depth, &ast), Ok(()));

    let expected_ast = comrak::parse_document(&doc_arena,
                                              &format!(r#"[not-link](not-link.html)
                                                          [link]({0}link.html)
                                                          ![img]({0}image.png)
                                                          [наб](https://nabijaczleweli.xyz)"#,
                                                      img_prepath),
                                              &MARKDOWN_OPTIONS);
    for (ov, ex) in ast.descendants().zip(expected_ast.descendants()) {
        assert_eq!(format!("{:?}", &ov.data.borrow().value), format!("{:?}", &ex.data.borrow().value));
    }
}
