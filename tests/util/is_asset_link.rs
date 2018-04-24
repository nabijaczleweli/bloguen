use bloguen::util::is_asset_link;


#[test]
fn yes() {
    assert!(is_asset_link("assets/link.html"));
    assert!(is_asset_link("assets/image.png"));
}

#[test]
fn no() {
    assert!(!is_asset_link("/assets/image.png"));
    assert!(!is_asset_link("//assets/image.png"));
    assert!(!is_asset_link("https://nabijaczleweli.xyz"));
}
