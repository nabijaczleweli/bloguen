use bloguen::ops::CenterOrder;
use bloguen::Error;


static VALID: &[&str] =
    &["backward", "backwarD", "backwaRd", "backwaRD", "backwArd", "backwArD", "backwARd", "backwARD", "backWard", "backWarD", "backWaRd", "backWaRD",
      "backWArd", "backWArD", "backWARd", "backWARD", "baCKward", "baCKwarD", "baCKwaRd", "baCKwaRD", "baCKwArd", "baCKwArD", "baCKwARd", "baCKwARD",
      "baCKWard", "baCKWarD", "baCKWaRd", "baCKWaRD", "baCKWArd", "baCKWArD", "baCKWARd", "baCKWARD", "baCkward", "baCkwarD", "baCkwaRd", "baCkwaRD",
      "baCkwArd", "baCkwArD", "baCkwARd", "baCkwARD", "baCkWard", "baCkWarD", "baCkWaRd", "baCkWaRD", "baCkWArd", "baCkWArD", "baCkWARd", "baCkWARD",
      "baCKward", "baCKwarD", "baCKwaRd", "baCKwaRD", "baCKwArd", "baCKwArD", "baCKwARd", "baCKwARD", "baCKWard", "baCKWarD", "baCKWaRd", "baCKWaRD",
      "baCKWArd", "baCKWArD", "baCKWARd", "baCKWARD", "bAckward", "bAckwarD", "bAckwaRd", "bAckwaRD", "bAckwArd", "bAckwArD", "bAckwARd", "bAckwARD",
      "bAckWard", "bAckWarD", "bAckWaRd", "bAckWaRD", "bAckWArd", "bAckWArD", "bAckWARd", "bAckWARD", "bACKward", "bACKwarD", "bACKwaRd", "bACKwaRD",
      "bACKwArd", "bACKwArD", "bACKwARd", "bACKwARD", "bACKWard", "bACKWarD", "bACKWaRd", "bACKWaRD", "bACKWArd", "bACKWArD", "bACKWARd", "bACKWARD",
      "bACkward", "bACkwarD", "bACkwaRd", "bACkwaRD", "bACkwArd", "bACkwArD", "bACkwARd", "bACkwARD", "bACkWard", "bACkWarD", "bACkWaRd", "bACkWaRD",
      "bACkWArd", "bACkWArD", "bACkWARd", "bACkWARD", "bACKward", "bACKwarD", "bACKwaRd", "bACKwaRD", "bACKwArd", "bACKwArD", "bACKwARd", "bACKwARD",
      "bACKWard", "bACKWarD", "bACKWaRd", "bACKWaRD", "bACKWArd", "bACKWArD", "bACKWARd", "bACKWARD", "Backward", "BackwarD", "BackwaRd", "BackwaRD",
      "BackwArd", "BackwArD", "BackwARd", "BackwARD", "BackWard", "BackWarD", "BackWaRd", "BackWaRD", "BackWArd", "BackWArD", "BackWARd", "BackWARD",
      "BaCKward", "BaCKwarD", "BaCKwaRd", "BaCKwaRD", "BaCKwArd", "BaCKwArD", "BaCKwARd", "BaCKwARD", "BaCKWard", "BaCKWarD", "BaCKWaRd", "BaCKWaRD",
      "BaCKWArd", "BaCKWArD", "BaCKWARd", "BaCKWARD", "BaCkward", "BaCkwarD", "BaCkwaRd", "BaCkwaRD", "BaCkwArd", "BaCkwArD", "BaCkwARd", "BaCkwARD",
      "BaCkWard", "BaCkWarD", "BaCkWaRd", "BaCkWaRD", "BaCkWArd", "BaCkWArD", "BaCkWARd", "BaCkWARD", "BaCKward", "BaCKwarD", "BaCKwaRd", "BaCKwaRD",
      "BaCKwArd", "BaCKwArD", "BaCKwARd", "BaCKwARD", "BaCKWard", "BaCKWarD", "BaCKWaRd", "BaCKWaRD", "BaCKWArd", "BaCKWArD", "BaCKWARd", "BaCKWARD",
      "BAckward", "BAckwarD", "BAckwaRd", "BAckwaRD", "BAckwArd", "BAckwArD", "BAckwARd", "BAckwARD", "BAckWard", "BAckWarD", "BAckWaRd", "BAckWaRD",
      "BAckWArd", "BAckWArD", "BAckWARd", "BAckWARD", "BACKward", "BACKwarD", "BACKwaRd", "BACKwaRD", "BACKwArd", "BACKwArD", "BACKwARd", "BACKwARD",
      "BACKWard", "BACKWarD", "BACKWaRd", "BACKWaRD", "BACKWArd", "BACKWArD", "BACKWARd", "BACKWARD", "BACkward", "BACkwarD", "BACkwaRd", "BACkwaRD",
      "BACkwArd", "BACkwArD", "BACkwARd", "BACkwARD", "BACkWard", "BACkWarD", "BACkWaRd", "BACkWaRD", "BACkWArd", "BACkWArD", "BACkWARd", "BACkWARD",
      "BACKward", "BACKwarD", "BACKwaRd", "BACKwaRD", "BACKwArd", "BACKwArD", "BACKwARd", "BACKwARD", "BACKWard", "BACKWarD", "BACKWaRd", "BACKWaRD",
      "BACKWArd", "BACKWArD", "BACKWARd", "BACKWARD"];
static INVALID: &[&str] = &["benlo", "бакворд"];


#[test]
fn from_str_ok() {
    for val in VALID {
        assert_eq!(val.parse(), Ok(CenterOrder::Backward));
    }
}

#[test]
fn from_str_err() {
    for val in INVALID {
        assert_eq!(val.parse::<CenterOrder>(),
                   Err(Error::Parse {
                       tp: "center order",
                       wher: "expected \"forward\" or \"backward\"".into(),
                       more: format!("\"{}\" invalid", val).into(),
                   }));
    }
}

#[test]
fn from_ok() {
    for val in VALID {
        assert_eq!(CenterOrder::from(val), Some(CenterOrder::Backward));
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
    assert_eq!(CenterOrder::Backward.name(), "backward");
}

#[test]
fn display() {
    assert_eq!(format!("{}", CenterOrder::Backward), "backward");
}

#[test]
fn transserialisation() {
    assert_eq!(CenterOrder::from(&format!("{}", CenterOrder::Backward)), Some(CenterOrder::Backward));
}
