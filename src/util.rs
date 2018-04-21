//! Module containing various utility functions.


use crc::crc32::checksum_ieee as crc32_ieee;
use comrak::ComrakOptions;
use chrono::NaiveTime;


lazy_static! {
    pub static ref MARKDOWN_OPTIONS: ComrakOptions = ComrakOptions {
        hardbreaks: true,
        ext_strikethrough: true,
        ext_table: true,
        ext_autolink: true,
        ext_tasklist: true,
        ext_header_ids: Some("".to_string()),
        ..ComrakOptions::default()
    };
}


/// Uppercase the first character of the supplied string.
///
/// Based on http://stackoverflow.com/a/38406885/2851815
///
/// # Examples
///
/// ```
/// # use bloguen::util::uppercase_first;
/// assert_eq!(uppercase_first("abolish"), "Abolish".to_string());
/// ```
pub fn uppercase_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Generate a reproducible post time from its name.
///
/// Works by IEEE-CRC32ing the name.
///
/// # Examples
///
/// ```
/// # extern crate bloguen;
/// # extern crate chrono;
/// # use bloguen::util::name_based_post_time;
/// # use chrono::NaiveTime;
/// assert_eq!(name_based_post_time("cursed device chain"), NaiveTime::from_hms(19, 03, 09));
/// ```
pub fn name_based_post_time(name: &str) -> NaiveTime {
    let digest = crc32_ieee(name.as_bytes());

    let hour = (digest & 0b11111) % 24;
    let minute = ((digest >> 5) & 0b111111) % 60;
    let second = ((digest >> (5 + 6)) & 0b111111) % 60;

    NaiveTime::from_hms(hour, minute, second)
}
