use comrak::{self, Arena as ComrakArena};
use std::fs::{self, File};
use std::env::temp_dir;
use chrono::NaiveTime;
use std::io::Write;
use bloguen::util;
use std::str;

mod parse_date_format_specifier;
mod parse_function_notation;
mod uppercase_first;
mod is_asset_link;
mod read_file;
mod bcp_47;


#[test]
fn name_based_post_time() {
    assert_eq!(util::name_based_post_time(""), NaiveTime::from_hms(00, 00, 00));
    assert_eq!(util::name_based_post_time("\x00"), NaiveTime::from_hms(13, 00, 29));
    assert_eq!(util::name_based_post_time("cursed device chain"), NaiveTime::from_hms(19, 03, 09));
}

/// Not quite sure how to test the non-UTF-8 error case, since the document is parsed from a UTF-8 string
#[test]
fn extract_links() {
    let doc_arena = ComrakArena::new();
    let ast = comrak::parse_document(&doc_arena,
                                     r#"[link](assets/link.html)
                                        ![img](assets/image.png)
                                        [наб](https://nabijaczleweli.xyz)"#,
                                     &util::MARKDOWN_OPTIONS);
    assert_eq!(util::extract_links(ast),
               Ok(vec!["assets/link.html".to_string(), "assets/image.png".to_string(), "https://nabijaczleweli.xyz".to_string()]));
}

/// Not quite sure how to test the non-UTF-8 error case, since the document is parsed from a UTF-8 string
#[test]
fn extract_actual_assets() {
    let root = temp_dir().join("bloguen-test").join("util-extract_actual_assets");
    fs::create_dir_all(root.join("images")).unwrap();
    File::create(root.join("images").join("i mage.png")).unwrap().write_all("images/i mage.png".as_bytes()).unwrap();
    File::create(root.join("link.html")).unwrap().write_all("link.html".as_bytes()).unwrap();

    let doc_arena = ComrakArena::new();
    let ast = comrak::parse_document(&doc_arena,
                                     r#"[not-link](not-link.html)
                                        [link](link.html)
                                        ![img](images/i%20mage.png)
                                        [наб](https://nabijaczleweli.xyz)"#,
                                     &util::MARKDOWN_OPTIONS);

    let actual_asset_links = util::extract_actual_assets(&root, &ast).unwrap();
    for (link, &expected) in actual_asset_links.into_iter().zip(["link.html", "images/i%20mage.png"].iter()) {
        assert_eq!(str::from_utf8(&link[..]), Ok(expected));
    }
}

// default_language() is untestable :v
