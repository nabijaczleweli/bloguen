use comrak::{self, Arena as ComrakArena};
use chrono::NaiveTime;
use bloguen::util;

mod uppercase_first;
mod is_asset_link;
mod read_file;


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

// default_language() is untestable :v
