use toml::from_str as from_toml_str;
use bloguen::ops::StyleElement;


#[derive(Deserialize)]
struct Data {
    pub data: StyleElement,
}

#[test]
fn invalid_class() {
    let res: Result<Data, _> = from_toml_str("[data]\nclass = 'helnlo'\ndata = '//nabijaczleweli.xyz/kaschism/assets/column.css'\n");
    assert_eq!(format!("{}", res.err().unwrap()),
               r#"invalid value: string "helnlo", expected "literal", "link", or "file" for key `data` at line 1 column 1"#);
}

#[test]
fn no_class() {
    let res: Result<Data, _> = from_toml_str("[data]\n\
                                              data = '//nabijaczleweli.xyz/kaschism/assets/column.css'\n");
    assert_eq!(format!("{}", res.err().unwrap()), r#"missing field `class` for key `data` at line 1 column 1"#);
}

#[test]
fn no_data() {
    let res: Result<Data, _> = from_toml_str("[data]\n\
                                              class = 'link'\n");
    assert_eq!(format!("{}", res.err().unwrap()), r#"missing field `data` for key `data` at line 1 column 1"#);
}

#[test]
fn dupe_class() {
    let res: Result<Data, _> = from_toml_str("[data]\nclass = 'link'\nclass = 'link'\ndata = '//nabijaczleweli.xyz/kaschism/assets/column.css'\n");
    assert_eq!(format!("{}", res.err().unwrap()), r#"duplicate field `class` for key `data` at line 1 column 1"#);
}

#[test]
fn dupe_data() {
    let res: Result<Data, _> = from_toml_str("[data]\nclass = 'link'\ndata = '//nabijaczleweli.xyz/kaschism/assets/column.css'\ndata = \
                                              '//nabijaczleweli.xyz/kaschism/assets/column.css'\n");
    assert_eq!(format!("{}", res.err().unwrap()), r#"duplicate field `data` for key `data` at line 1 column 1"#);
}


#[test]
fn unknown_field() {
    let res: Result<Data, _> = from_toml_str("[data]\n\
                                              helnlo = 'link'\n");
    assert_eq!(format!("{}", res.err().unwrap()),
               r#"unknown field `helnlo`, expected `class` or `data` for key `data` at line 1 column 1"#);
}
