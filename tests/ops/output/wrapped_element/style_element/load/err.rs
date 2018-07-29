use bloguen::ops::StyleElement;
use std::fs::{self, File};
use std::env::temp_dir;
use bloguen::Error;
use std::io::Write;


#[test]
fn path_nonexistant() {
    let root = temp_dir().join("bloguen-test").join("ops-output-wrapped_element-style_element-load-err-path_nonexistant");
    let _ = fs::remove_dir_all(root.join("style"));

    let mut dt = StyleElement::from_path("style/henlo/../common.css");
    let bkp = dt.clone();

    assert_eq!(dt.load(&("$ROOT".to_string(), root)),
               Err(Error::FileNotFound {
                   who: "file style element",
                   path: "$ROOT/style/henlo/../common.css".into(),
               }));
    assert_eq!(dt, bkp);
}

#[test]
fn path_non_utf8() {
    let root = temp_dir().join("bloguen-test").join("ops-output-wrapped_element-style_element-load-err-path_non_utf8");
    let _ = fs::create_dir_all(root.join("style"));
    File::create(root.join("style").join("common.css"))
        .unwrap()
        .write_all(&[0xC3, 0x28, 0xA0, 0xA1, 0xE2, 0x28, 0xA1, 0xE2, 0x82, 0x28, 0xF0, 0x28, 0x8C, 0xBC, 0xF0, 0x90, 0x28, 0xBC, 0xF0, 0x28, 0x8C, 0x28])
        .unwrap();

    let mut dt = StyleElement::from_path("style/henlo/../common.css");
    let bkp = dt.clone();

    assert_eq!(dt.load(&("$ROOT".to_string(), root)),
               Err(Error::Parse {
                   tp: "UTF-8 string",
                   wher: "file style element".into(),
                   more: None,
               }));
    assert_eq!(dt, bkp);
}
