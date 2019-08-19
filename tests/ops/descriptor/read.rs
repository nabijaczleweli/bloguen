use bloguen::ops::{BlogueDescriptorIndex, BlogueDescriptor, MachineDataKind, ScriptElement, StyleElement, CenterOrder};
use std::fs::{self, File};
use std::env::temp_dir;
use std::io::Write;
use bloguen::Error;


#[cfg(target_os = "windows")]
const ALT_SLASH: char = '\\';
#[cfg(not(target_os = "windows"))]
const ALT_SLASH: char = '/';

#[cfg(target_os = "windows")]
const ALT_SLASH_ESC: &str = "\\\\";
#[cfg(not(target_os = "windows"))]
const ALT_SLASH_ESC: &str = "/";


#[test]
fn ok_all_specified() {
    let root = temp_dir().join("bloguen-test").join("ops-descriptor-read-ok_all_specified");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("templates")).unwrap();

    File::create(root.join("blogue.toml"))
        .unwrap()
        .write_all(format!("name = \"Блогг\"\n\
                            author = \"nabijaczleweli\"\n\
                            header = \"templates/head\"\n\
                            footer = \"templates{0}foot\"\n\
                            asset_dir = \"{0}/as{0}set/dir\"\n\
                            language = \"pl\"\n\
                            styles = [\"link://nabijaczleweli.xyz/kaschism/assets/column.css\",\n\
                                      \"literal:.indented {{ text-indent: 1em; }}\"]\n\
                            \n\
                            [index]\n\
                            generate = true\n
                            header = \"templates/idx_head\"\n\
                            center = \"templates/idx_центр\"\n\
                            footer = \"templates{0}idx_foot\"\n\
                            order = \"backward\"\n\
                            styles = [\"file:common.css\"]\n\
                            scripts = [\"literal:console.log(\\\"adenosinetriphosphate\\\");\"]\n\
                            data = {{ preferred-system = \"capitalism\" }}\n\
                            \n\
                            [[scripts]]\n\
                            class = \"link\"\n\
                            data = \"/content/assets/syllable.js\"\n\
                            \n\
                            [[scripts]]\n\
                            class = \"file\"\n\
                            data = \"MathJax-config.js\"\n\
                            \n\
                            [machine_data]\n\
                            JSON = \"metadata/json/\"\n\
                            \n\
                            [data]\n\
                            preferred-system = \"communism\"\n",
                           ALT_SLASH_ESC)
            .as_bytes())
        .unwrap();
    File::create(root.join("templates").join("head")).unwrap();
    File::create(root.join("templates").join("foot")).unwrap();
    File::create(root.join("templates").join("idx_head")).unwrap();
    File::create(root.join("templates").join("idx_центр")).unwrap();
    File::create(root.join("templates").join("idx_foot")).unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/".to_string(), root.clone())),
               Ok(BlogueDescriptor {
                   name: "Блогг".to_string(),
                   author: Some("nabijaczleweli".to_string()),
                   header_file: ("$ROOT/templates/head".to_string(), root.join("templates").join("head")),
                   footer_file: (format!("$ROOT/templates{}foot", ALT_SLASH), root.join("templates").join("foot")),
                   asset_dir_override: Some("as/set/dir/".to_string()),
                   index: Some(BlogueDescriptorIndex {
                       header_file: ("$ROOT/templates/idx_head".to_string(), root.join("templates").join("idx_head")),
                       center_file: ("$ROOT/templates/idx_центр".to_string(), root.join("templates").join("idx_центр")),
                       footer_file: (format!("$ROOT/templates{}idx_foot", ALT_SLASH), root.join("templates").join("idx_foot")),
                       center_order: CenterOrder::Backward,
                       styles: vec![StyleElement::from_path("common.css")],
                       scripts: vec![ScriptElement::from_literal("console.log(\"adenosinetriphosphate\");")],
                       data: vec![("preferred-system".to_string(), "capitalism".to_string())].into_iter().collect(),
                   }),
                   machine_data: vec![(MachineDataKind::Json, "metadata/json/".to_string())].into_iter().collect(),
                   language: Some("pl".parse().unwrap()),
                   styles: vec![StyleElement::from_link("//nabijaczleweli.xyz/kaschism/assets/column.css"),
                                StyleElement::from_literal(".indented { text-indent: 1em; }")],
                   scripts: vec![ScriptElement::from_link("/content/assets/syllable.js"), ScriptElement::from_path("MathJax-config.js")],
                   data: vec![("preferred-system".to_string(), "communism".to_string())].into_iter().collect(),
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
                   author: None,
                   header_file: ("$ROOT/header.html".to_string(), root.join("header.html")),
                   footer_file: ("$ROOT/footer.htm".to_string(), root.join("footer.htm")),
                   asset_dir_override: None,
                   machine_data: vec![].into_iter().collect(),
                   language: None,
                   styles: vec![],
                   scripts: vec![],
                   index: None,
                   data: vec![].into_iter().collect(),
               }));
}

#[test]
fn ok_induced_index() {
    let root = temp_dir().join("bloguen-test").join("ops-descriptor-read-ok_induced_index");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("templates")).unwrap();

    File::create(root.join("blogue.toml"))
        .unwrap()
        .write_all("name = \"Блогг\"\n\
                    \n\
                    [index]\n\
                    generate = true\n"
            .as_bytes())
        .unwrap();
    File::create(root.join("header.html")).unwrap();
    File::create(root.join("footer.htm")).unwrap();
    File::create(root.join("index_header.html")).unwrap();
    File::create(root.join("index_center.html")).unwrap();
    File::create(root.join("index_footer.htm")).unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/".to_string(), root.clone())),
               Ok(BlogueDescriptor {
                   name: "Блогг".to_string(),
                   author: None,
                   header_file: ("$ROOT/header.html".to_string(), root.join("header.html")),
                   footer_file: ("$ROOT/footer.htm".to_string(), root.join("footer.htm")),
                   asset_dir_override: None,
                   machine_data: vec![].into_iter().collect(),
                   language: None,
                   styles: vec![],
                   scripts: vec![],
                   index: Some(BlogueDescriptorIndex {
                       header_file: ("$ROOT/index_header.html".to_string(), root.join("index_header.html")),
                       center_file: ("$ROOT/index_center.html".to_string(), root.join("index_center.html")),
                       footer_file: ("$ROOT/index_footer.htm".to_string(), root.join("index_footer.htm")),
                       center_order: CenterOrder::Forward,
                       styles: vec![],
                       scripts: vec![],
                       data: vec![].into_iter().collect(),
                   }),
                   data: vec![].into_iter().collect(),
               }));
}

#[test]
fn ok_induced_idx() {
    let root = temp_dir().join("bloguen-test").join("ops-descriptor-read-ok_induced_idx");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("templates")).unwrap();

    File::create(root.join("blogue.toml"))
        .unwrap()
        .write_all("name = \"Блогг\"\n\
                    \n\
                    [index]\n\
                    generate = true\n"
            .as_bytes())
        .unwrap();
    File::create(root.join("header.html")).unwrap();
    File::create(root.join("footer.htm")).unwrap();
    File::create(root.join("idx_header.html")).unwrap();
    File::create(root.join("idx_center.htm")).unwrap();
    File::create(root.join("idx_footer.htm")).unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/".to_string(), root.clone())),
               Ok(BlogueDescriptor {
                   name: "Блогг".to_string(),
                   author: None,
                   header_file: ("$ROOT/header.html".to_string(), root.join("header.html")),
                   footer_file: ("$ROOT/footer.htm".to_string(), root.join("footer.htm")),
                   asset_dir_override: None,
                   machine_data: vec![].into_iter().collect(),
                   language: None,
                   styles: vec![],
                   scripts: vec![],
                   index: Some(BlogueDescriptorIndex {
                       header_file: ("$ROOT/idx_header.html".to_string(), root.join("idx_header.html")),
                       center_file: ("$ROOT/idx_center.htm".to_string(), root.join("idx_center.htm")),
                       footer_file: ("$ROOT/idx_footer.htm".to_string(), root.join("idx_footer.htm")),
                       center_order: CenterOrder::Forward,
                       styles: vec![],
                       scripts: vec![],
                       data: vec![].into_iter().collect(),
                   }),
                   data: vec![].into_iter().collect(),
               }));
}

#[test]
fn invalid_machine_data_empty_path() {
    let root = temp_dir().join("bloguen-test").join("ops-descriptor-read-invalid_machine_data_empty_path");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("templates")).unwrap();

    File::create(root.join("blogue.toml"))
        .unwrap()
        .write_all("name = \"Блогг\"\n\
                    \n\
                    [machine_data]\n\
                    JSON = \"\"\n"
            .as_bytes())
        .unwrap();
    File::create(root.join("header.html")).unwrap();
    File::create(root.join("footer.htm")).unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/".to_string(), root.clone())),
               Err(Error::Parse {
                   tp: "path chunk",
                   wher: "blogue descriptor".into(),
                   more: "JSON subdir selector empty".into(),
               }));
}

#[test]
fn invalid_machine_data_slash_path() {
    let root = temp_dir().join("bloguen-test").join("ops-descriptor-read-invalid_machine_data_slash_path");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("templates")).unwrap();

    File::create(root.join("blogue.toml"))
        .unwrap()
        .write_all("name = \"Блогг\"\n\
                    \n\
                    [machine_data]\n\
                    JSON = \"/\"\n"
            .as_bytes())
        .unwrap();
    File::create(root.join("header.html")).unwrap();
    File::create(root.join("footer.htm")).unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/".to_string(), root.clone())),
               Err(Error::Parse {
                   tp: "path chunk",
                   wher: "blogue descriptor".into(),
                   more: "JSON subdir selector empty".into(),
               }));
}

#[test]
fn invalid_machine_data_invalid_kind() {
    let root = temp_dir().join("bloguen-test").join("ops-descriptor-read-invalid_machine_data_invalid_kind");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("templates")).unwrap();

    File::create(root.join("blogue.toml"))
        .unwrap()
        .write_all("name = \"Блогг\"\n\
                    \n\
                    [machine_data]\n\
                    JSON = \"\"\n"
            .as_bytes())
        .unwrap();
    File::create(root.join("header.html")).unwrap();
    File::create(root.join("footer.htm")).unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/".to_string(), root.clone())),
               Err(Error::Parse {
                   tp: "path chunk",
                   wher: "blogue descriptor".into(),
                   more: "JSON subdir selector empty".into(),
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
                   desc: "blogue descriptor".into(),
                   errors: "Failed to parse BCP-47 language tag for language specifier: \"en*\" invalid for key `language` at line 1 column 1".into()
               }));
}

#[test]
fn invalid_style_element() {
    let root = temp_dir().join("bloguen-test").join("ops-descriptor-read-invalid_style_element");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("templates")).unwrap();

    File::create(root.join("blogue.toml"))
        .unwrap()
        .write_all("styles = [\"henlo:benlo\"]".as_bytes())
        .unwrap();
    File::create(root.join("header.html")).unwrap();
    File::create(root.join("footer.htm")).unwrap();

    assert_eq!(BlogueDescriptor::read(&("$ROOT/".to_string(), root.clone())),
               Err(Error::FileParsingFailed {
                   desc: "blogue descriptor".into(),
                   errors: "invalid value: string \"henlo\", expected \"literal\", \"link\", or \"file\" for key `styles` at line 1 column 11".into(),
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
                   desc: "blogue descriptor".into(),
                   op: "read",
                   more: "not UTF-8".into(),
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
                   desc: "blogue descriptor".into(),
                   errors: "expected a right bracket, found a newline at line 1 column 13".into(),
               }));
}
