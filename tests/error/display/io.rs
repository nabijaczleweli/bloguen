use bloguen::Error;


#[test]
fn normal_non_e() {
    assert_eq!(Error::Io {
                       desc: "input file".into(),
                       op: "read",
                       more: "stream ended".into(),
                   }
                   .to_string(),
               "Reading input file failed: stream ended.");
}

#[test]
fn normal_e() {
    assert_eq!(Error::Io {
                       desc: "output file".into(),
                       op: "create",
                       more: "stream ended".into(),
                   }
                   .to_string(),
               "Creating output file failed: stream ended.");
}

#[test]
fn single_non_e() {
    assert_eq!(Error::Io {
                       desc: "input file".into(),
                       op: "C",
                       more: "stream ended".into(),
                   }
                   .to_string(),
               "Cing input file failed: stream ended.");
}

#[test]
fn single_e() {
    assert_eq!(Error::Io {
                       desc: "input file".into(),
                       op: "e",
                       more: "stream ended".into(),
                   }
                   .to_string(),
               "ing input file failed: stream ended.");
}

#[test]
fn empty() {
    assert_eq!(Error::Io {
                       desc: "input file".into(),
                       op: "",
                       more: "stream ended".into(),
                   }
                   .to_string(),
               "ing input file failed: stream ended.");
}
