use bloguen::ops::BlogueDescriptor;
use std::fs::{self, File};
use std::env::temp_dir;
use std::io::Write;
use bloguen::Error;


#[test]
fn ok_all_specified() {
    let root = temp_dir().join("bloguen-test").join("ops-descriptor-read-ok_all_specified");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("templates")).unwrap();

    File::create(root.join("blogue.toml"))
        .unwrap()
        .write_all("name = \"Блогг\"\n\
                    header = \"templates/head\"\n\
                    footer = \"templates\\\\foot\"\n\
                    language = \"pl\"\n"
            .as_bytes())
        .unwrap();
    File::create(root.join("templates").join("head")).unwrap();
    File::create(root.join("templates").join("foot")).unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/".to_string(), root.clone())),
               Ok(BlogueDescriptor {
                   name: "Блогг".to_string(),
                   header_file: ("$ROOT/templates/head".to_string(), root.join("templates").join("head")),
                   footer_file: ("$ROOT/templates\\foot".to_string(), root.join("templates").join("foot")),
                   language: Some("pl".parse().unwrap()),
               }));
}

#[test]
fn ok_induced() {
    let root = temp_dir().join("bloguen-test").join("ops-descriptor-read-ok_induced");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("templates")).unwrap();

    File::create(root.join("blogue.toml")).unwrap().write_all("name = \"Блогг\"\n".as_bytes()).unwrap();
    File::create(root.join("header.html")).unwrap();
    File::create(root.join("footer.htm")).unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/".to_string(), root.clone())),
               Ok(BlogueDescriptor {
                   name: "Блогг".to_string(),
                   header_file: ("$ROOT/header.html".to_string(), root.join("header.html")),
                   footer_file: ("$ROOT/footer.htm".to_string(), root.join("footer.htm")),
                   language: None,
               }));
}

#[test]
fn invalid_language() {
    let root = temp_dir().join("bloguen-test").join("ops-descriptor-read-invalid_language");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("templates")).unwrap();

    File::create(root.join("blogue.toml"))
        .unwrap()
        .write_all("name = \"Блогг\"\n\
                    language = \"en*\"\n"
            .as_bytes())
        .unwrap();
    File::create(root.join("header.html")).unwrap();
    File::create(root.join("footer.htm")).unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/".to_string(), root.clone())),
               Err(Error::FileParsingFailed {
                   desc: "blogue descriptor",
                   errors: Some("Failed to parse BCP-47 language tag for language specifier: \"en*\" invalid for key `language`".into()),
               }));
}

#[test]
fn descriptor_not_found() {
    let root = temp_dir().join("bloguen-test").join("ops-descriptor-read-descriptor_not_found");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/".to_string(), root)),
               Err(Error::FileNotFound {
                   who: "blogue descriptor",
                   path: "$ROOT/blogue.toml".into(),
               }));
}

#[test]
fn non_utf8() {
    let root = temp_dir().join("bloguen-test").join("ops-descriptor-read-non_utf8");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();

    // https://stackoverflow.com/a/3886015/2851815
    File::create(root.join("blogue.toml"))
        .unwrap()
        .write_all(&[0xC3, 0x28, 0xA0, 0xA1, 0xE2, 0x28, 0xA1, 0xE2, 0x82, 0x28, 0xF0, 0x28, 0x8C, 0xBC, 0xF0, 0x90, 0x28, 0xBC, 0xF0, 0x28, 0x8C, 0x28])
        .unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/".to_string(), root)),
               Err(Error::Io {
                   desc: "blogue descriptor",
                   op: "read",
                   more: Some("not UTF-8".into()),
               }));
}

#[test]
fn invalid_toml() {
    let root = temp_dir().join("bloguen-test").join("ops-descriptor-read-invalid_toml");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();

    File::create(root.join("blogue.toml"))
        .unwrap()
        .write_all("[description\n".as_bytes())
        .unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/".into(), root)),
               Err(Error::FileParsingFailed {
                   desc: "blogue descriptor",
                   errors: Some("expected a right bracket, found a newline at line 1".into()),
               }));
}
