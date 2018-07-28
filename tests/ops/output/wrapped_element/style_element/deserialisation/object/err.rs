use toml::from_str as from_toml_str;
use bloguen::ops::StyleElement;


#[derive(Deserialize)]
struct Data {
    pub data: StyleElement,
}

#[test]
fn invalid_class() {
    let res: Result<Data, _> = from_toml_str("[data]\n\
                                              class = 'helnlo'\n\
                                              data = '//nabijaczlewli.xyz/kaschism/assets/column.css'\n");
    assert_eq!(format!("{}", res.err().unwrap()), r#"invalid value: string "helnlo", expected "literal", "link", or "file" for key `data`"#);
}

#[test]
fn no_class() {
    let res: Result<Data, _> = from_toml_str("[data]\n\
                                              data = '//nabijaczlewli.xyz/kaschism/assets/column.css'\n");
    assert_eq!(format!("{}", res.err().unwrap()), r#"missing field `class` for key `data`"#);
}

#[test]
fn no_data() {
    let res: Result<Data, _> = from_toml_str("[data]\n\
                                              class = 'link'\n");
    assert_eq!(format!("{}", res.err().unwrap()), r#"missing field `data` for key `data`"#);
}

#[test]
fn dupe_class() {
    let res: Result<Data, _> = from_toml_str("[data]\n\
                                              class = 'link'\n\
                                              class = 'link'\n\
                                              data = '//nabijaczlewli.xyz/kaschism/assets/column.css'\n");
    assert_eq!(format!("{}", res.err().unwrap()), r#"duplicate field `class` for key `data`"#);
}

#[test]
fn dupe_data() {
    let res: Result<Data, _> = from_toml_str("[data]\n\
                                              class = 'link'\n\
                                              data = '//nabijaczlewli.xyz/kaschism/assets/column.css'\n\
                                              data = '//nabijaczlewli.xyz/kaschism/assets/column.css'\n");
    assert_eq!(format!("{}", res.err().unwrap()), r#"duplicate field `data` for key `data`"#);
}


#[test]
fn unknown_field() {
    let res: Result<Data, _> = from_toml_str("[data]\n\
                                              helnlo = 'link'\n");
    assert_eq!(format!("{}", res.err().unwrap()), r#"unknown field `helnlo`, expected `class` or `data` for key `data`"#);
}
