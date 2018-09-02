use bloguen::ops::MachineDataKind;
use std::str::FromStr;


#[test]
fn json_parse() {
    assert_eq!(MachineDataKind::from_str("json"), Ok(MachineDataKind::Json));
    assert_eq!(MachineDataKind::from_str("jsoN"), Ok(MachineDataKind::Json));
    assert_eq!(MachineDataKind::from_str("jsOn"), Ok(MachineDataKind::Json));
    assert_eq!(MachineDataKind::from_str("jsON"), Ok(MachineDataKind::Json));
    assert_eq!(MachineDataKind::from_str("jSon"), Ok(MachineDataKind::Json));
    assert_eq!(MachineDataKind::from_str("jSoN"), Ok(MachineDataKind::Json));
    assert_eq!(MachineDataKind::from_str("jSOn"), Ok(MachineDataKind::Json));
    assert_eq!(MachineDataKind::from_str("jSON"), Ok(MachineDataKind::Json));
    assert_eq!(MachineDataKind::from_str("Json"), Ok(MachineDataKind::Json));
    assert_eq!(MachineDataKind::from_str("JsoN"), Ok(MachineDataKind::Json));
    assert_eq!(MachineDataKind::from_str("JsOn"), Ok(MachineDataKind::Json));
    assert_eq!(MachineDataKind::from_str("JsON"), Ok(MachineDataKind::Json));
    assert_eq!(MachineDataKind::from_str("JSon"), Ok(MachineDataKind::Json));
    assert_eq!(MachineDataKind::from_str("JSoN"), Ok(MachineDataKind::Json));
    assert_eq!(MachineDataKind::from_str("JSOn"), Ok(MachineDataKind::Json));
    assert_eq!(MachineDataKind::from_str("JSON"), Ok(MachineDataKind::Json));
}

#[test]
fn json_stringify() {
    assert_eq!(MachineDataKind::Json.to_string(), "JSON");
}
