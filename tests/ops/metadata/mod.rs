mod read_or_default;

use std::collections::BTreeMap;
use bloguen::ops::PostMetadata;
use std::default::Default;


#[test]
fn default() {
    assert_eq!(PostMetadata::default(),
               PostMetadata {
                   language: None,
                   author: None,
                   tags: vec![],
                   styles: vec![],
                   scripts: vec![],
                   data: BTreeMap::new(),
               });
}
