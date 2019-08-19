use bloguen::ops::FeedType;
use bloguen::Error;


static VALID: &[&str] = &["rss", "rsS", "rSs", "rSS", "Rss", "RsS", "RSs", "RSS"];
static INVALID: &[&str] = &["benlo", "форвард"];


#[test]
fn from_str_ok() {
    for val in VALID {
        assert_eq!(val.parse(), Ok(FeedType::Rss));
    }
}

#[test]
fn from_str_err() {
    for val in INVALID {
        assert_eq!(val.parse::<FeedType>(),
                   Err(Error::Parse {
                       tp: "feed type",
                       wher: "expected \"RSS\" or \"Atom\"".into(),
                       more: format!("\"{}\" invalid", val).into(),
                   }));
    }
}

#[test]
fn from_ok() {
    for val in VALID {
        assert_eq!(FeedType::from(val), Some(FeedType::Rss));
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
    assert_eq!(FeedType::Rss.name(), "RSS");
}

#[test]
fn display() {
    assert_eq!(format!("{}", FeedType::Rss), "RSS");
}

#[test]
fn transserialisation() {
    assert_eq!(FeedType::from(&format!("{}", FeedType::Rss)), Some(FeedType::Rss));
}
