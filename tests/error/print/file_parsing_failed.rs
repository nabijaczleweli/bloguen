use bloguen::Error;


#[test]
fn no_error() {
    let mut out = Vec::new();
    Error::FileParsingFailed {
            desc: "blogue descriptor",
            errors: None,
        }
        .print_error(&mut out);
    assert_eq!(out,
               "Failed to parse blogue descriptor.\n".as_bytes());
}

#[test]
fn with_error() {
    let mut out = Vec::new();
    Error::FileParsingFailed {
            desc: "blogue descriptor",
            errors: Some("unexpected eof encountered".to_string()),
        }
        .print_error(&mut out);
    assert_eq!(out,
               "Failed to parse blogue descriptor: unexpected eof encountered.\n".as_bytes());
}
