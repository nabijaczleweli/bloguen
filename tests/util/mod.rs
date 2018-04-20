use chrono::NaiveTime;
use bloguen::util;

mod uppercase_first;


#[test]
fn name_based_post_time() {
    assert_eq!(util::name_based_post_time(""), NaiveTime::from_hms(00, 00, 00));
    assert_eq!(util::name_based_post_time("\x00"), NaiveTime::from_hms(13, 00, 29));
    assert_eq!(util::name_based_post_time("cursed device chain"), NaiveTime::from_hms(19, 03, 09));
}
