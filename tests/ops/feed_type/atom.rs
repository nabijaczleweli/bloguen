use bloguen::ops::FeedType;
use bloguen::Error;


static VALID: &[&str] = &["atom", "atoM", "atOm", "atOM", "aTom", "aToM", "aTOm", "aTOM", "Atom", "AtoM", "AtOm", "AtOM", "ATom", "AToM", "ATOm", "ATOM"];
static INVALID: &[&str] = &["benlo", "бакворд"];


#[test]
fn from_str_ok() {
    for val in VALID {
        assert_eq!(val.parse(), Ok(FeedType::Atom));
    }
}

#[test]
fn from_str_err() {
    for val in INVALID {
        assert_eq!(val.parse::<FeedType>(),
                   Err(Error::Parse {
                       tp: "machine data specifier",
                       wher: "expected \"RSS\" or \"Atom\"".into(),
                       more: Some(format!("\"{}\" invalid", val).into()),
                   }));
    }
}

#[test]
fn from_ok() {
    for val in VALID {
        assert_eq!(FeedType::from(val), Some(FeedType::Atom));
    }
}

#[test]
fn from_err() {
    for val in INVALID {
        assert_eq!(FeedType::from(val), None);
    }
}


#[test]
fn name() {
    assert_eq!(FeedType::Atom.name(), "Atom");
}

#[test]
fn display() {
    assert_eq!(format!("{}", FeedType::Atom), "Atom");
}

#[test]
fn transserialisation() {
    assert_eq!(FeedType::from(&format!("{}", FeedType::Atom)), Some(FeedType::Atom));
}
