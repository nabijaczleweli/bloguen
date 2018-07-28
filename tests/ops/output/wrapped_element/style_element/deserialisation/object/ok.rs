use toml::from_str as from_toml_str;
use bloguen::ops::StyleElement;


#[derive(Deserialize)]
struct Data {
    pub data: StyleElement,
}

#[test]
fn link() {
    let Data { data } = from_toml_str("[data]\n\
                                       class = 'link'\n\
                                       data = '//nabijaczlewli.xyz/kaschism/assets/column.css'\n").unwrap();
    assert_eq!(data, StyleElement::from_link("//nabijaczlewli.xyz/kaschism/assets/column.css"));
}

#[test]
fn literal() {
    let Data { data } = from_toml_str("[data]\n\
                                       class = 'literal'\n\
                                       data = '.indented { text-indent: 1em; }'").unwrap();
    assert_eq!(data, StyleElement::from_literal(".indented { text-indent: 1em; }"));
}

#[test]
fn file() {
    let Data { data } = from_toml_str("[data]\n\
                                       class = 'file'\n\
                                       data = 'common.css'").unwrap();
    assert_eq!(data, StyleElement::from_path("common.css"));
}
