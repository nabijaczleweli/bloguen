use bloguen::Error;


#[test]
fn normal_non_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file".into(),
            op: "read",
            more: None,
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(), "Reading input file failed.\n".to_string());
}

#[test]
fn normal_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "output file".into(),
            op: "create",
            more: None,
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(), "Creating output file failed.\n".to_string());
}

#[test]
fn single_non_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file".into(),
            op: "C",
            more: None,
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(), "Cing input file failed.\n".to_string());
}

#[test]
fn single_e() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file".into(),
            op: "e",
            more: None,
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(), "ing input file failed.\n".to_string());
}

#[test]
fn empty() {
    let mut out = Vec::new();
    Error::Io {
            desc: "input file".into(),
            op: "",
            more: None,
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(), "ing input file failed.\n".to_string());
}
