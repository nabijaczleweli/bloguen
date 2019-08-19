use bloguen::ops::StyleElement;
use std::fs::{self, File};
use std::env::temp_dir;
use bloguen::Error;
use std::io::Write;


#[test]
fn path_nonexistant() {
    let root = temp_dir().join("bloguen-test").join("ops-output-wrapped_element-style_element-load-from_file-path_nonexistant");
    let _ = fs::remove_dir_all(root.join("style"));

    assert_eq!(StyleElement::from_file(&("$ROOT/style/common.css".to_string(), root.join("style/common.css"))),
               Err(Error::FileNotFound {
                   who: "literal style element from path",
                   path: "$ROOT/style/common.css".into(),
               }));
}

#[test]
fn path_non_utf8() {
    let root = temp_dir().join("bloguen-test").join("ops-output-wrapped_element-style_element-load-from_file-path_non_utf8");
    let _ = fs::create_dir_all(root.join("style"));
    File::create(root.join("style").join("common.css"))
        .unwrap()
        .write_all(&[0xC3, 0x28, 0xA0, 0xA1, 0xE2, 0x28, 0xA1, 0xE2, 0x82, 0x28, 0xF0, 0x28, 0x8C, 0xBC, 0xF0, 0x90, 0x28, 0xBC, 0xF0, 0x28, 0x8C, 0x28])
        .unwrap();

    assert_eq!(StyleElement::from_file(&("$ROOT/style/common.css".to_string(), root.join("style/common.css"))),
               Err(Error::Parse {
                   tp: "UTF-8 string",
                   wher: "literal style element from path".into(),
                   more: "stream did not contain valid UTF-8".into(),
               }));
}
