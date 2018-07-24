use bloguen::Error;


#[test]
fn io() {
    assert_eq!(Error::Io {
                       desc: "".into(),
                       op: "",
                       more: None,
                   }
                   .exit_value(),
               1);
    assert_eq!(Error::Io {
                       desc: "".into(),
                       op: "",
                       more: Some("".into()),
                   }
                   .exit_value(),
               1);
}

#[test]
fn parse() {
    assert_eq!(Error::Parse {
                       tp: "",
                       wher: "".into(),
                       more: None,
                   }
                   .exit_value(),
               2);
    assert_eq!(Error::Parse {
                       tp: "",
                       wher: "".into(),
                       more: Some("".into()),
                   }
                   .exit_value(),
               2);
}

#[test]
fn file_not_found() {
    assert_eq!(Error::FileNotFound {
                       who: "",
                       path: "".into(),
                   }
                   .exit_value(),
               3);
}

#[test]
fn wrong_file_state() {
    assert_eq!(Error::WrongFileState {
                       what: "",
                       path: "".into(),
                   }
                   .exit_value(),
               4);
}

#[test]
fn file_parsing_failed() {
    assert_eq!(Error::FileParsingFailed {
                       desc: "".into(),
                       errors: None,
                   }
                   .exit_value(),
               5);
    assert_eq!(Error::FileParsingFailed {
                       desc: "".into(),
                       errors: Some("".into()),
                   }
                   .exit_value(),
               5);
}
