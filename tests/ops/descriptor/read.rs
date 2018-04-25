use bloguen::ops::BlogueDescriptor;
use std::fs::{self, File};
use std::env::temp_dir;
use std::io::Write;
use bloguen::Error;


#[test]
fn ok() {
    let td = temp_dir().join("bloguen-test").join("ops-descriptor-read-ok");
    fs::create_dir_all(&td).unwrap();

    let tf = td.join("blogue.toml");
    let _ = fs::remove_file(&tf);

    File::create(&tf).unwrap().write_all(r#"name = "Блогг""#.as_bytes()).unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/blogue.toml".to_string(), tf)), Ok(BlogueDescriptor { name: "Блогг".to_string() }));
}

#[test]
fn not_found() {
    let td = temp_dir().join("bloguen-test").join("ops-descriptor-read-not_found");
    fs::create_dir_all(&td).unwrap();

    let tf = td.join("blogue.toml");
    let _ = fs::remove_file(&tf);

    assert_eq!(BlogueDescriptor::read(&("$ROOT/blogue.toml".to_string(), tf)),
               Err(Error::FileNotFound {
                   who: "blogue descriptor",
                   path: "$ROOT/blogue.toml".to_string(),
               }));
}

#[test]
fn non_utf8() {
    let td = temp_dir().join("bloguen-test").join("ops-descriptor-read-non_utf8");
    fs::create_dir_all(&td).unwrap();

    let tf = td.join("blogue.toml");
    let _ = fs::remove_file(&tf);

    // https://stackoverflow.com/a/3886015/2851815
    File::create(&tf)
        .unwrap()
        .write_all(&[0xC3, 0x28, 0xA0, 0xA1, 0xE2, 0x28, 0xA1, 0xE2, 0x82, 0x28, 0xF0, 0x28, 0x8C, 0xBC, 0xF0, 0x90, 0x28, 0xBC, 0xF0, 0x28, 0x8C, 0x28])
        .unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/blogue.toml".to_string(), tf)),
               Err(Error::Io {
                   desc: "blogue descriptor",
                   op: "read",
                   more: Some("not UTF-8".to_string()),
               }));
}

#[test]
fn invalid_toml() {
    let td = temp_dir().join("bloguen-test").join("ops-descriptor-read-invalid_toml");
    fs::create_dir_all(&td).unwrap();

    let tf = td.join("blogue.toml");
    let _ = fs::remove_file(&tf);

    File::create(&tf)
        .unwrap()
        .write_all("[description\n".as_bytes())
        .unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/blogue.toml".to_string(), tf)),
               Err(Error::FileParsingFailed {
                   desc: "blogue descriptor",
                   errors: Some("expected a right bracket, found a newline at line 1".to_string()),
               }));
}
