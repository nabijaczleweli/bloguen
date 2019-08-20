use bloguen::ops::MachineDataKind;
use bloguen::Error;


static VALID: &[&str] = &["json", "jsoN", "jsOn", "jsON", "jSon", "jSoN", "jSOn", "jSON", "Json", "JsoN", "JsOn", "JsON", "JSon", "JSoN", "JSOn", "JSON"];
static INVALID: &[&str] = &["benlo", "ЙСОН"];


#[test]
fn from_str_ok() {
    for val in VALID {
        assert_eq!(val.parse(), Ok(MachineDataKind::Json));
    }
}

#[test]
fn from_str_err() {
    for val in INVALID {
        assert_eq!(val.parse::<MachineDataKind>(),
                   Err(Error::Parse {
                       tp: "machine data specifier",
                       wher: "expected \"JSON\"".into(),
                       more: format!("\"{}\" invalid", val).into(),
                   }));
    }
}

#[test]
fn from_ok() {
    for val in VALID {
        assert_eq!(MachineDataKind::from(val), Some(MachineDataKind::Json));
    }
}

#[test]
fn from_err() {
    for val in INVALID {
        assert_eq!(MachineDataKind::from(val), None);
    }
}


#[test]
fn name() {
    assert_eq!(MachineDataKind::Json.name(), "JSON");
}

#[test]
fn extension() {
    assert_eq!(MachineDataKind::Json.extension(), "json");
}

#[test]
fn display() {
    assert_eq!(format!("{}", MachineDataKind::Json), "JSON");
}

#[test]
fn transserialisation() {
    assert_eq!(MachineDataKind::from(&format!("{}", MachineDataKind::Json)), Some(MachineDataKind::Json));
}
