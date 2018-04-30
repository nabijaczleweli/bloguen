use bloguen::util::BCP_47;


// Examples from http://schneegans.de/lv
#[test]
fn ok() {
    assert!(BCP_47.is_match("de"));
    assert!(BCP_47.is_match("de-CH"));
    assert!(BCP_47.is_match("de-DE-1901"));
    assert!(BCP_47.is_match("es-419"));
    assert!(BCP_47.is_match("sl-IT-nedis"));
    assert!(BCP_47.is_match("en-US-boont"));
    assert!(BCP_47.is_match("mn-Cyrl-MN"));
    assert!(BCP_47.is_match("x-fr-CH"));
    assert!(BCP_47.is_match("en-GB-boont-r-extended-sequence-x-private"));
    assert!(BCP_47.is_match("sr-Cyrl"));
    assert!(BCP_47.is_match("sr-Latn"));
    assert!(BCP_47.is_match("hy-Latn-IT-arevela"));
    assert!(BCP_47.is_match("zh-TW"));
}

#[test]
fn bad() {
    assert!(!BCP_47.is_match("en_GB"));
    assert!(!BCP_47.is_match("en*"));
}
