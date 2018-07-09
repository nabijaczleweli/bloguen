use chrono::format::{StrftimeItems, Fixed, Item};
use bloguen::util::parse_date_format_specifier;


#[test]
fn rfc2822() {
    assert_eq!(parse_date_format_specifier("rfc2822"), Some(vec![Item::Fixed(Fixed::RFC2822)].into()));
    assert_eq!(parse_date_format_specifier("RFC2822"), Some(vec![Item::Fixed(Fixed::RFC2822)].into()));
    assert_eq!(parse_date_format_specifier("rfc_2822"), Some(vec![Item::Fixed(Fixed::RFC2822)].into()));
    assert_eq!(parse_date_format_specifier("RFC_2822"), Some(vec![Item::Fixed(Fixed::RFC2822)].into()));
}

#[test]
fn rfc3339() {
    assert_eq!(parse_date_format_specifier("rfc3339"), Some(vec![Item::Fixed(Fixed::RFC3339)].into()));
    assert_eq!(parse_date_format_specifier("RFC3339"), Some(vec![Item::Fixed(Fixed::RFC3339)].into()));
    assert_eq!(parse_date_format_specifier("rfc_3339"), Some(vec![Item::Fixed(Fixed::RFC3339)].into()));
    assert_eq!(parse_date_format_specifier("RFC_3339"), Some(vec![Item::Fixed(Fixed::RFC3339)].into()));
}

#[test]
fn custom_ok() {
    assert_eq!(parse_date_format_specifier("\"%Y %B %d\""), Some(StrftimeItems::new("%Y %B %d").collect()));
    assert_eq!(parse_date_format_specifier("\"%s%:z\""), Some(StrftimeItems::new("%s%:z").collect()));
}

#[test]
fn unmatched() {
    assert_eq!(parse_date_format_specifier("benlo"), None);
    assert_eq!(parse_date_format_specifier("\"%s%:z"), None);
    assert_eq!(parse_date_format_specifier("%s%:z\""), None);
    assert_eq!(parse_date_format_specifier("%s%:z"), None);
}
