use toml::from_str as from_toml_str;
use bloguen::ops::StyleElement;


#[derive(Deserialize)]
struct Data {
    pub data: StyleElement,
}

#[test]
fn link() {
    let Data { data } = from_toml_str("[data]\nclass = 'link'\ndata = '//nabijaczleweli.xyz/kaschism/assets/column.css'\n").unwrap();
    assert_eq!(data, StyleElement::from_link("//nabijaczleweli.xyz/kaschism/assets/column.css"));
}

#[test]
fn literal() {
    let Data { data } = from_toml_str("[data]\nclass = 'literal'\ndata = '.indented { text-indent: 1em; }'").unwrap();
    assert_eq!(data, StyleElement::from_literal(".indented { text-indent: 1em; }"));
}

#[test]
fn file() {
    let Data { data } = from_toml_str("[data]\nclass = 'file'\ndata = 'common.css'").unwrap();
    assert_eq!(data, StyleElement::from_path("common.css"));
}
