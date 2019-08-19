mod io;

use bloguen::Error;


#[test]
fn parse() {
    assert_eq!(Error::Parse {
                       tp: "datetime",
                       wher: "post descriptor".into(),
                       more: "not RFC3339".into(),
                   }
                   .to_string(),
               "Failed to parse datetime for post descriptor: not RFC3339.");
}

#[test]
fn file_not_found() {
    assert_eq!(Error::FileNotFound {
                       who: "thumbnail",
                       path: "file/that/does/not.exist".into(),
                   }
                   .to_string(),
               "File file/that/does/not.exist for thumbnail not found.");
}

#[test]
fn wrong_file_state() {
    assert_eq!(Error::WrongFileState {
                       what: "actually a file",
                       path: "file/that/does/not.exist".into(),
                   }
                   .to_string(),
               "File file/that/does/not.exist is not actually a file.");
}

#[test]
fn file_parsing_failed() {
    assert_eq!(Error::FileParsingFailed {
                       desc: "blogue descriptor".into(),
                       errors: "unexpected eof encountered".into(),
                   }
                   .to_string(),
               "Failed to parse blogue descriptor: unexpected eof encountered.");
}
