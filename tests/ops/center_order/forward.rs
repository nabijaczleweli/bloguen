use bloguen::ops::CenterOrder;
use bloguen::Error;


static VALID: &[&str] = &["forward", "forwarD", "forwaRd", "forwaRD", "forwArd", "forwArD", "forwARd", "forwARD", "forWard", "forWarD", "forWaRd", "forWaRD",
                          "forWArd", "forWArD", "forWARd", "forWARD", "foRward", "foRwarD", "foRwaRd", "foRwaRD", "foRwArd", "foRwArD", "foRwARd", "foRwARD",
                          "foRWard", "foRWarD", "foRWaRd", "foRWaRD", "foRWArd", "foRWArD", "foRWARd", "foRWARD", "fOrward", "fOrwarD", "fOrwaRd", "fOrwaRD",
                          "fOrwArd", "fOrwArD", "fOrwARd", "fOrwARD", "fOrWard", "fOrWarD", "fOrWaRd", "fOrWaRD", "fOrWArd", "fOrWArD", "fOrWARd", "fOrWARD",
                          "fORward", "fORwarD", "fORwaRd", "fORwaRD", "fORwArd", "fORwArD", "fORwARd", "fORwARD", "fORWard", "fORWarD", "fORWaRd", "fORWaRD",
                          "fORWArd", "fORWArD", "fORWARd", "fORWARD", "Forward", "ForwarD", "ForwaRd", "ForwaRD", "ForwArd", "ForwArD", "ForwARd", "ForwARD",
                          "ForWard", "ForWarD", "ForWaRd", "ForWaRD", "ForWArd", "ForWArD", "ForWARd", "ForWARD", "FoRward", "FoRwarD", "FoRwaRd", "FoRwaRD",
                          "FoRwArd", "FoRwArD", "FoRwARd", "FoRwARD", "FoRWard", "FoRWarD", "FoRWaRd", "FoRWaRD", "FoRWArd", "FoRWArD", "FoRWARd", "FoRWARD",
                          "FOrward", "FOrwarD", "FOrwaRd", "FOrwaRD", "FOrwArd", "FOrwArD", "FOrwARd", "FOrwARD", "FOrWard", "FOrWarD", "FOrWaRd", "FOrWaRD",
                          "FOrWArd", "FOrWArD", "FOrWARd", "FOrWARD", "FORward", "FORwarD", "FORwaRd", "FORwaRD", "FORwArd", "FORwArD", "FORwARd", "FORwARD",
                          "FORWard", "FORWarD", "FORWaRd", "FORWaRD", "FORWArd", "FORWArD", "FORWARd", "FORWARD"];
static INVALID: &[&str] = &["benlo", "форвард"];


#[test]
fn from_str_ok() {
    for val in VALID {
        assert_eq!(val.parse(), Ok(CenterOrder::Forward));
    }
}

#[test]
fn from_str_err() {
    for val in INVALID {
        assert_eq!(val.parse::<CenterOrder>(),
                   Err(Error::Parse {
                       tp: "machine data specifier",
                       wher: "expected \"forward\" or \"backward\"".into(),
                       more: Some(format!("\"{}\" invalid", val).into()),
                   }));
    }
}

#[test]
fn from_ok() {
    for val in VALID {
        assert_eq!(CenterOrder::from(val), Some(CenterOrder::Forward));
    }
}

#[test]
fn from_err() {
    for val in INVALID {
        assert_eq!(CenterOrder::from(val), None);
    }
}


#[test]
fn name() {
    assert_eq!(CenterOrder::Forward.name(), "forward");
}

#[test]
fn display() {
    assert_eq!(format!("{}", CenterOrder::Forward), "forward");
}

#[test]
fn transserialisation() {
    assert_eq!(CenterOrder::from(&format!("{}", CenterOrder::Forward)), Some(CenterOrder::Forward));
}
