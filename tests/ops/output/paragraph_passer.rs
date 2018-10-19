use bloguen::ops::ParagraphPasser;
use std::io::Write;
use std::str;


static PARAGRAPHS_ALL: &str = include_str!("../../../test-data/paragraphs/all.html");


macro_rules! all_test {
    ($fname:ident, $num:expr, $fnum:expr) => {
        #[test]
        fn $fname() {
            let mut out = vec![];
            ParagraphPasser::new(&mut out, $num).write_all(PARAGRAPHS_ALL.as_bytes()).unwrap();
            assert_eq!(str::from_utf8(&out).unwrap(), include_str!(concat!("../../../test-data/paragraphs/", $fnum, ".html")));
        }
    }
}


all_test!(all_0, 0, "00");
all_test!(all_1, 1, "01");
all_test!(all_2, 2, "02");
all_test!(all_3, 3, "03");
all_test!(all_4, 4, "04");
all_test!(all_5, 5, "05");
all_test!(all_6, 6, "06");
all_test!(all_7, 7, "07");
all_test!(all_8, 8, "08");
all_test!(all_9, 9, "09");
all_test!(all_10, 10, "10");
all_test!(all_11, 11, "11");
all_test!(all_12, 12, "12");
all_test!(all_13, 13, "13");
all_test!(all_14, 14, "14");
all_test!(all_15, 15, "15");
all_test!(all_16, 16, "16");
all_test!(all_17, 17, "17");
all_test!(all_18, 18, "18");
all_test!(all_19, 19, "19");
all_test!(all_20, 20, "20");
all_test!(all_21, 21, "all");
