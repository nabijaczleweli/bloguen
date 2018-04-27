use bloguen::util::read_file;
use std::fs::{self, File};
use std::env::temp_dir;
use std::io::Write;
use bloguen::Error;


#[test]
fn ok() {
    let root = temp_dir().join("bloguen-test").join("ops-util-read_file-ok");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();

    let data = r#"
      .file_icon:before {
        content: url('{file_icon}');
        margin-right: 0.5em;
      }

      .file_binary_icon:before {
        content: url('{file_binary_icon}');
        margin-right: 0.5em;
      }

      .file_image_icon:before {
        content: url('{file_image_icon}');
        margin-right: 0.5em;
      }

      .file_text_icon:before {
        content: url('{file_text_icon}');
        margin-right: 0.5em;
      }

      .back_arrow_icon:before {
        content: url('{back_arrow_icon}');
        margin-right: 0.5em;
      }
      "#;
    File::create(root.join("data")).unwrap().write_all(data.as_bytes()).unwrap();
    assert_eq!(read_file(&("$ROOT/data".to_string(), root.join("data")), "data"), Ok(data.to_string()));
}

#[test]
fn not_found() {
    let root = temp_dir().join("bloguen-test").join("ops-util-read_file-not_found");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();

    assert_eq!(read_file(&("$ROOT/data".to_string(), root.join("data")), "data"),
               Err(Error::FileNotFound {
                   who: "data",
                   path: "$ROOT/data".to_string(),
               }));
}

// Not sure how to test an I/O Error

#[test]
fn not_utf8() {
    let root = temp_dir().join("bloguen-test").join("ops-util-read_file-not_utf8");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();

    File::create(root.join("data"))
        .unwrap()
        .write_all(&[0xC3, 0x28, 0xA0, 0xA1, 0xE2, 0x28, 0xA1, 0xE2, 0x82, 0x28, 0xF0, 0x28, 0x8C, 0xBC, 0xF0, 0x90, 0x28, 0xBC, 0xF0, 0x28, 0x8C, 0x28])
        .unwrap();
    assert_eq!(read_file(&("$ROOT/data".to_string(), root.join("data")), "data"),
               Err(Error::Parse {
                   tp: "UTF-8 string",
                   wher: "data",
                   more: None,
               }));
}
