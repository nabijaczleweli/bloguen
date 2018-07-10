use bloguen::util::parse_function_notation;


#[test]
fn ok() {
    assert_eq!(parse_function_notation("date()"), Some(("date", vec![])));
    assert_eq!(parse_function_notation("date(post)"), Some(("date", vec!["post"])));
    assert_eq!(parse_function_notation("date(\"%Y %B %d\", post)"),
               Some(("date", vec!["\"%Y %B %d\"", "post"])));
}

#[test]
fn no_name() {
    assert_eq!(parse_function_notation("()"), None);
    assert_eq!(parse_function_notation("("), None);
    assert_eq!(parse_function_notation("(post)"), None);
    assert_eq!(parse_function_notation("(\"%Y %B %d\", post)"), None);
}

#[test]
fn no_right_paren() {
    assert_eq!(parse_function_notation("date("), Some(("date", vec![])));
    assert_eq!(parse_function_notation("date(post"), Some(("date", vec![])));
    assert_eq!(parse_function_notation("date(\"%Y %B %d\", post"), Some(("date", vec![])));
}
