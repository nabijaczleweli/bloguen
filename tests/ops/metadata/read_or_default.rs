use bloguen::ops::{ScriptElement, StyleElement, PostMetadata};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::default::Default;
use std::env::temp_dir;
use std::io::Write;
use bloguen::Error;


#[test]
fn ok_all_specified() {
    let post_root = temp_dir().join("bloguen-test").join("ops-metadata-read_or_default-ok_all_specified");
    let _ = fs::remove_dir_all(&post_root);
    fs::create_dir_all(&post_root).unwrap();

    File::create(post_root.join("metadata.toml"))
        .unwrap()
        .write_all("language = \"pl\"\n\
                    author = \"nabijaczleweli\"\n\
                    styles = [\"link://nabijaczleweli.xyz/kaschism/assets/column.css\",\n\
                              \"literal:.indented { text-indent: 1em; }\"]\n\
                    \n\
                    [[scripts]]\n\
                    class = \"link\"\n\
                    data = \"/content/assets/syllable.js\"\n\
                    \n\
                    [[scripts]]\n\
                    class = \"file\"\n\
                    data = \"MathJax-config.js\"\n\
                    \n\
                    [data]\n\
                    desc = \"Każdy koniec to nowy początek [PL]\"\n\
                    communism = \"yass, queen\"\n"
            .as_bytes())
        .unwrap();

    assert_eq!(PostMetadata::read_or_default(&("$POST_ROOT/".to_string(), post_root)),
               Ok(PostMetadata {
                   language: Some("pl".parse().unwrap()),
                   author: Some("nabijaczleweli".to_string()),
                   tags: vec![],
                   styles: vec![StyleElement::from_link("//nabijaczleweli.xyz/kaschism/assets/column.css"),
                                StyleElement::from_literal(".indented { text-indent: 1em; }")],
                   scripts: vec![ScriptElement::from_link("/content/assets/syllable.js"), ScriptElement::from_path("MathJax-config.js")],
                   data: vec![("desc".to_string(), "Każdy koniec to nowy początek [PL]".to_string()), ("communism".to_string(), "yass, queen".to_string())]
                       .into_iter()
                       .collect(),
               }));
}

#[test]
fn ok_no_data() {
    let post_root = temp_dir().join("bloguen-test").join("ops-metadata-read_or_default-ok_no_data");
    let _ = fs::remove_dir_all(&post_root);
    fs::create_dir_all(&post_root).unwrap();

    File::create(post_root.join("metadata.toml"))
        .unwrap()
        .write_all("language = \"pl\"\n\
                    author = \"nabijaczleweli\"\n"
            .as_bytes())
        .unwrap();

    assert_eq!(PostMetadata::read_or_default(&("$POST_ROOT/".to_string(), post_root)),
               Ok(PostMetadata {
                   language: Some("pl".parse().unwrap()),
                   author: Some("nabijaczleweli".to_string()),
                   tags: vec![],
                   styles: vec![],
                   scripts: vec![],
                   data: BTreeMap::new(),
               }));
}

#[test]
fn ok_just_data() {
    let post_root = temp_dir().join("bloguen-test").join("ops-metadata-read_or_default-ok_just_data");
    let _ = fs::remove_dir_all(&post_root);
    fs::create_dir_all(&post_root).unwrap();

    File::create(post_root.join("metadata.toml"))
        .unwrap()
        .write_all("styles = [\"link://nabijaczleweli.xyz/kaschism/assets/column.css\",\n\
                              \"literal:.indented { text-indent: 1em; }\"]\n\
                    \n\
                    [[scripts]]\n\
                    class = \"link\"\n\
                    data = \"/content/assets/syllable.js\"\n\
                    \n\
                    [[scripts]]\n\
                    class = \"file\"\n\
                    data = \"MathJax-config.js\"\n\
                    \n\
                    [data]\n\
                    desc = \"Każdy koniec to nowy początek [PL]\"\n\
                    communism = \"yass, queen\"\n"
            .as_bytes())
        .unwrap();

    assert_eq!(PostMetadata::read_or_default(&("$POST_ROOT/".to_string(), post_root)),
               Ok(PostMetadata {
                   language: None,
                   author: None,
                   tags: vec![],
                   styles: vec![StyleElement::from_link("//nabijaczleweli.xyz/kaschism/assets/column.css"),
                                StyleElement::from_literal(".indented { text-indent: 1em; }")],
                   scripts: vec![ScriptElement::from_link("/content/assets/syllable.js"), ScriptElement::from_path("MathJax-config.js")],
                   data: vec![("desc".to_string(), "Każdy koniec to nowy początek [PL]".to_string()), ("communism".to_string(), "yass, queen".to_string())]
                       .into_iter()
                       .collect(),
               }));
}

#[test]
fn invalid_style_element() {
    let post_root = temp_dir().join("bloguen-test").join("ops-metadata-read_or_default-invalid_style_element");
    let _ = fs::remove_dir_all(&post_root);
    fs::create_dir_all(post_root.join("templates")).unwrap();

    File::create(post_root.join("metadata.toml"))
        .unwrap()
        .write_all("styles = [\"henlo:benlo\"]".as_bytes())
        .unwrap();
    File::create(post_root.join("header.html")).unwrap();
    File::create(post_root.join("footer.htm")).unwrap();

    assert_eq!(PostMetadata::read_or_default(&("$POST_ROOT/".to_string(), post_root)),
               Err(Error::FileParsingFailed {
                   desc: "post metadata".into(),
                   errors: Some("invalid value: string \"henlo\", expected \"literal\", \"link\", or \"file\" for key `styles`".into()),
               }));
}

#[test]
fn invalid_language() {
    let post_root = temp_dir().join("bloguen-test").join("ops-metadata-read_or_default-invalid_language");
    let _ = fs::remove_dir_all(&post_root);
    fs::create_dir_all(post_root.join("templates")).unwrap();

    File::create(post_root.join("metadata.toml"))
        .unwrap()
        .write_all("name = \"Блогг\"\n\
                    language = \"en*\"\n"
            .as_bytes())
        .unwrap();
    File::create(post_root.join("header.html")).unwrap();
    File::create(post_root.join("footer.htm")).unwrap();

    assert_eq!(PostMetadata::read_or_default(&("$POST_ROOT/".to_string(), post_root)),
               Err(Error::FileParsingFailed {
                   desc: "post metadata".into(),
                   errors: Some("Failed to parse BCP-47 language tag for language specifier: \"en*\" invalid for key `language`".into()),
               }));
}

#[test]
fn metadata_not_found() {
    let post_root = temp_dir().join("bloguen-test").join("ops-metadata-read_or_default-metadata_not_found");

    assert_eq!(PostMetadata::read_or_default(&("$POST_ROOT/".to_string(), post_root)), Ok(Default::default()));
}

#[test]
fn non_utf8() {
    let post_root = temp_dir().join("bloguen-test").join("ops-metadata-read_or_default-non_utf8");
    let _ = fs::remove_dir_all(&post_root);
    fs::create_dir_all(&post_root).unwrap();

    // https://stackoverflow.com/a/3886015/2851815
    File::create(post_root.join("metadata.toml"))
        .unwrap()
        .write_all(&[0xC3, 0x28, 0xA0, 0xA1, 0xE2, 0x28, 0xA1, 0xE2, 0x82, 0x28, 0xF0, 0x28, 0x8C, 0xBC, 0xF0, 0x90, 0x28, 0xBC, 0xF0, 0x28, 0x8C, 0x28])
        .unwrap();

    assert_eq!(PostMetadata::read_or_default(&("$POST_ROOT/".to_string(), post_root)),
               Err(Error::Io {
                   desc: "post metadata".into(),
                   op: "read",
                   more: Some("not UTF-8".into()),
               }));
}

#[test]
fn invalid_toml() {
    let post_root = temp_dir().join("bloguen-test").join("ops-metadata-read_or_default-invalid_toml");
    let _ = fs::remove_dir_all(&post_root);
    fs::create_dir_all(&post_root).unwrap();

    File::create(post_root.join("metadata.toml"))
        .unwrap()
        .write_all("[description\n".as_bytes())
        .unwrap();

    assert_eq!(PostMetadata::read_or_default(&("$POST_ROOT/".to_string(), post_root)),
               Err(Error::FileParsingFailed {
                   desc: "post metadata".into(),
                   errors: Some("expected a right bracket, found a newline at line 1".into()),
               }));
}
