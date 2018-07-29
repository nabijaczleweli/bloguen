use bloguen::ops::StyleElement;
use std::fs::{self, File};
use std::env::temp_dir;
use std::io::Write;


static COLUMN_CSS: &str = include_str!("../../../../../../assets/column.css");


#[test]
fn link() {
    let root = temp_dir().join("bloguen-test").join("ops-output-wrapped_element-style_element-load-ok-link");
    let _ = fs::create_dir_all(&root);

    let mut dt = StyleElement::from_link("//nabijaczlewli.xyz/kaschism/assets/column.css");
    let bkp = dt.clone();

    assert_eq!(dt.load(&("$ROOT".to_string(), root)), Ok(()));
    assert_eq!(dt, bkp);
}

#[test]
fn literal() {
    let root = temp_dir().join("bloguen-test").join("ops-output-wrapped_element-style_element-load-ok-literal");
    let _ = fs::create_dir_all(&root);

    let mut dt = StyleElement::from_link(".indented { text-indent: 1em; }");
    let bkp = dt.clone();

    assert_eq!(dt.load(&("$ROOT".to_string(), root)), Ok(()));
    assert_eq!(dt, bkp);
}

#[test]
fn path() {
    let root = temp_dir().join("bloguen-test").join("ops-output-wrapped_element-style_element-load-ok-path");
    let _ = fs::create_dir_all(root.join("style"));
    File::create(root.join("style").join("common.css")).unwrap().write_all(COLUMN_CSS.as_bytes()).unwrap();

    let mut dt = StyleElement::from_path("style/henlo/../common.css");

    assert_eq!(dt.load(&("$ROOT".to_string(), root)), Ok(()));
    assert_eq!(dt, StyleElement::from_literal(COLUMN_CSS));
}
