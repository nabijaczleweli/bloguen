mod file_parsing_failed;
mod parse;
mod io;

use bloguen::Error;


#[test]
fn file_not_found() {
    let mut out = Vec::new();
    Error::FileNotFound {
            who: "thumbnail",
            path: "file/that/does/not.exist".to_string(),
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(),
               "File file/that/does/not.exist for thumbnail not found.\n".to_string());
}

#[test]
fn wrong_file_state() {
    let mut out = Vec::new();
    Error::WrongFileState {
            what: "actually a file",
            path: "file/that/does/not.exist".to_string(),
        }
        .print_error(&mut out);
    assert_eq!(out.iter().map(|&i| i as char).collect::<String>(),
               "File file/that/does/not.exist is not actually a file.\n".to_string());
}
