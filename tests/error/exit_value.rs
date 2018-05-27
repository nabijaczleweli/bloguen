use bloguen::Error;
use std::path::PathBuf;


#[test]
fn io() {
    assert_eq!(Error::Io {
                       desc: "",
                       op: "",
                       more: None,
                   }
                   .exit_value(),
               1);
    assert_eq!(Error::Io {
                       desc: "",
                       op: "",
                       more: Some("".to_string()),
                   }
                   .exit_value(),
               1);
}

#[test]
fn parse() {
    assert_eq!(Error::Parse {
                       tp: "",
                       wher: "",
                       more: None,
                   }
                   .exit_value(),
               2);
    assert_eq!(Error::Parse {
                       tp: "",
                       wher: "",
                       more: Some(""),
                   }
                   .exit_value(),
               2);
}

#[test]
fn file_not_found() {
    assert_eq!(Error::FileNotFound {
                       who: "",
                       path: PathBuf::new(),
                   }
                   .exit_value(),
               3);
}

#[test]
fn wrong_file_state() {
    assert_eq!(Error::WrongFileState {
                       what: "",
                       path: PathBuf::new(),
                   }
                   .exit_value(),
               4);
}

#[test]
fn file_parsing_failed_() {
    assert_eq!(Error::FileParsingFailed {
                       desc: "",
                       errors: None,
                   }
                   .exit_value(),
               5);
    assert_eq!(Error::FileParsingFailed {
                       desc: "",
                       errors: Some("".to_string()),
                   }
                   .exit_value(),
               5);
}
