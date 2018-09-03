use bloguen::ops::MachineDataKind;


#[test]
fn json_parse() {
    assert_eq!("json".parse(), Ok(MachineDataKind::Json));
    assert_eq!("jsoN".parse(), Ok(MachineDataKind::Json));
    assert_eq!("jsOn".parse(), Ok(MachineDataKind::Json));
    assert_eq!("jsON".parse(), Ok(MachineDataKind::Json));
    assert_eq!("jSon".parse(), Ok(MachineDataKind::Json));
    assert_eq!("jSoN".parse(), Ok(MachineDataKind::Json));
    assert_eq!("jSOn".parse(), Ok(MachineDataKind::Json));
    assert_eq!("jSON".parse(), Ok(MachineDataKind::Json));
    assert_eq!("Json".parse(), Ok(MachineDataKind::Json));
    assert_eq!("JsoN".parse(), Ok(MachineDataKind::Json));
    assert_eq!("JsOn".parse(), Ok(MachineDataKind::Json));
    assert_eq!("JsON".parse(), Ok(MachineDataKind::Json));
    assert_eq!("JSon".parse(), Ok(MachineDataKind::Json));
    assert_eq!("JSoN".parse(), Ok(MachineDataKind::Json));
    assert_eq!("JSOn".parse(), Ok(MachineDataKind::Json));
    assert_eq!("JSON".parse(), Ok(MachineDataKind::Json));
}

#[test]
fn json_stringify() {
    assert_eq!(MachineDataKind::Json.to_string(), "JSON");
}
