use toml::from_str as from_toml_str;
use bloguen::ops::StyleElement;


#[derive(Deserialize)]
struct Data {
    pub data: StyleElement,
}

#[test]
fn invalid_class() {
    let res: Result<Data, _> = from_toml_str("data = 'helnlo://nabijaczleweli.xyz/kaschism/assets/column.css'");
    assert_eq!(format!("{}", res.err().unwrap()),
               r#"invalid value: string "helnlo", expected "literal", "link", or "file" for key `data`"#);
}

#[test]
fn invalid_specless() {
    let res: Result<Data, _> = from_toml_str("data = '.indented { text-indent: 1em; }'");
    assert_eq!(format!("{}", res.err().unwrap()),
               r#"invalid value: string ".indented { text-indent", expected "literal", "link", or "file" for key `data`"#);
}
