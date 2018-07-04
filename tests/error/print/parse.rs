use bloguen::Error;


#[test]
fn no_more() {
    let mut out = Vec::new();
    Error::Parse {
            tp: "e-mail",
            wher: "post descriptor",
            more: None,
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(), "Failed to parse e-mail for post descriptor.\n");
}

#[test]
fn more() {
    let mut out = Vec::new();
    Error::Parse {
            tp: "datetime",
            wher: "post descriptor",
            more: Some("not RFC3339".to_string()),
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(),
               "Failed to parse datetime for post descriptor: not RFC3339.\n");
}
