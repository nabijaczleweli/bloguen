use bloguen::ops::StyleElement;
use std::fs::{self, File};
use std::env::temp_dir;
use std::io::Write;

mod err;


static COLUMN_CSS: &str = include_str!("../../../../../../assets/column.css");


#[test]
fn ok() {
    let root = temp_dir().join("bloguen-test").join("ops-output-wrapped_element-style_element-from_file-ok");
    let _ = fs::create_dir_all(root.join("style"));
    File::create(root.join("style").join("common.css")).unwrap().write_all(COLUMN_CSS.as_bytes()).unwrap();

    assert_eq!(StyleElement::from_file(&("$ROOT/style/henlo/../common.css".to_string(), root.join("style/henlo/../common.css"))),
               Ok(StyleElement::from_literal(COLUMN_CSS)));
}
