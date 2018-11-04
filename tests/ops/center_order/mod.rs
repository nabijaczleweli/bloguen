use bloguen::ops::CenterOrder;


mod backward;
mod forward;


fn default() {
    assert_eq!(CenterOrder::default(), CenterOrder::Forward);
}
