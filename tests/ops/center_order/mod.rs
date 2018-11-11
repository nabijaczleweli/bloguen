use bloguen::ops::CenterOrder;


mod backward;
mod forward;


#[test]
fn default() {
    assert_eq!(CenterOrder::default(), CenterOrder::Forward);
}
