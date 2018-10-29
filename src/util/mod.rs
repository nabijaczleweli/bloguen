//! Module containing various utility functions.


#[cfg(target_os = "windows")]
mod windows;
mod polywrite;
#[cfg(not(target_os = "windows"))]
mod non_windows;

use chrono::format::{StrftimeItems as StrftimeFormatItems, Fixed as FixedTimeFormatItem, Item as TimeFormatItem};
use comrak::nodes::{NodeValue as ComrakNodeValue, AstNode as ComrakAstNode};
use chrono::{FixedOffset, NaiveTime, DateTime, TimeZone, Offset};
use safe_transmute::guarded_transmute_to_bytes_pod;
use std::io::{ErrorKind as IoErrorKind, Read};
use crc::crc32::checksum_ieee as crc32_ieee;
use std::path::{self, PathBuf, Path};
use self::super::ops::LanguageTag;
use rand::{SeedableRng, Rng};
use rand::prng::XorShiftRng;
use comrak::ComrakOptions;
use self::super::Error;
use std::borrow::Cow;
use std::{cmp, str};
use std::fs::File;
use regex::Regex;
use url::Url;

#[cfg(target_os = "windows")]
use self::windows::{current_username_impl, default_language_impl};
#[cfg(not(target_os = "windows"))]
use self::non_windows::{current_username_impl, default_language_impl};

pub use self::polywrite::PolyWrite;


include!(concat!(env!("OUT_DIR"), "/words.rs"));

lazy_static! {
    /// Options to use for parsing Markdown.
    pub static ref MARKDOWN_OPTIONS: ComrakOptions = ComrakOptions {
        hardbreaks: true,
        ext_strikethrough: true,
        ext_table: true,
        ext_autolink: true,
        ext_tasklist: true,
        ext_header_ids: Some("".to_string()),
        ..ComrakOptions::default()
    };

    /// Regex to use for parsing a BCP47 language tag.
    ///
    /// Stolen from http://stackoverflow.com/a/7036171/2851815
    pub static ref BCP_47: Regex = Regex::new(include_str!("../../assets/bcp47.regex").trim()).unwrap();

    /// The default `en-GB` language tag.
    pub static ref LANGUAGE_EN_GB: LanguageTag = "en-GB".parse().unwrap();
}

/// Current version of `bloguen`.
pub static BLOGUEN_VERSION: &str = env!("CARGO_PKG_VERSION");


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

/// Generate a reproducible blogue author from its name.
///
/// Works by seeding an [XOR-shift](../../rand/prng/struct.XorShiftRng.html) with the IEEE-CRC32d name.
///
/// The generated name has a `.2` probability of including a middle portion, then a `.25` probability of it being full,
/// as opposed to just an initial.
///
/// # Examples
///
/// ```
/// # use bloguen::util::name_based_full_name;
/// assert_eq!(name_based_full_name("Блогг"),          "Specifically Shopper");
/// assert_eq!(name_based_full_name("Blogue"),         "Very Turban");
///
/// assert_eq!(name_based_full_name("Ben's Blog"),     "Properly P. Postbox");
/// assert_eq!(name_based_full_name("Benjojo's Blog"), "Why W. Wannabe");
///
/// assert_eq!(name_based_full_name("Inquiescence"),   "Always Basket Baritone");
/// ```
pub fn name_based_full_name(name: &str) -> String {
    let digest = crc32_ieee(name.as_bytes());
    let mut seed = [0u8; 16];
    if cfg!(target_endian = "little") {
        guarded_transmute_to_bytes_pod(&digest)
            .iter()
            .cycle()
            .zip(seed.iter_mut())
            .for_each(|(d, s)| *s = *d);
    } else {
        guarded_transmute_to_bytes_pod(&digest)
            .iter()
            .rev()
            .cycle()
            .zip(seed.iter_mut())
            .for_each(|(d, s)| *s = *d);
    }
    let mut rng = XorShiftRng::from_seed(seed);

    let first_name = rng.choose(ADVERBS).unwrap();
    let last_name = rng.choose(NOUNS).unwrap();
    if rng.gen_bool(0.2) {
        let middle_name = rng.choose(ADJECTIVES).unwrap();
        if rng.gen_bool(0.25) {
            format!("{} {} {}", first_name, middle_name, last_name)
        } else {
            format!("{} {}. {}", first_name, middle_name.chars().next().unwrap(), last_name)
        }
    } else {
        format!("{} {}", first_name, last_name)
    }
}

/// Get list of all links in the specified AST.
///
/// # Examples
///
/// ```
/// # extern crate bloguen;
/// # extern crate comrak;
/// # use bloguen::util::{MARKDOWN_OPTIONS, extract_links};
/// let doc_arena = comrak::Arena::new();
/// let ast =
///     comrak::parse_document(&doc_arena, r#"[link](assets/link.html)
///                                           ![img](assets/image.png)"#, &MARKDOWN_OPTIONS);
/// assert_eq!(extract_links(ast),
///            Ok(vec!["assets/link.html".to_string(), "assets/image.png".to_string()]));
/// ```
pub fn extract_links<'a>(ast: &'a ComrakAstNode<'a>) -> Result<Vec<String>, Error> {
    let mut out = vec![];

    for n in ast.descendants() {
        match n.data.borrow().value {
            ComrakNodeValue::Link(ref link) |
            ComrakNodeValue::Image(ref link) => {
                out.push(String::from_utf8(link.url.clone()).map_err(|_| {
                        Error::Parse {
                            tp: "UTF-8 string",
                            wher: "URL list".into(),
                            more: None,
                        }
                    })?);
            }
            _ => {}
        }
    }

    Ok(out)
}

/// Apply the asset override to the specified AST.
///
/// The `depth` argument specifies how many `"../"` segments are how deep the output file is relative to root parent directory.
///
/// Remember: this funxion *modifies the passed-in AST*, despite the reference being immutable.
///
/// # Examples
///
/// Given the following directory layout:
///
/// ```plaintext
/// $ROOT/
///   image.png
/// ```
///
/// The following holds:
///
/// ```
/// # extern crate bloguen;
/// # extern crate comrak;
/// # use bloguen::ops::{BlogueDescriptorIndex, BlogueDescriptor, MachineDataKind, ScriptElement, StyleElement,
/// #                    CenterOrder};
/// # use bloguen::util::{extract_actual_assets, MARKDOWN_OPTIONS};
/// # use std::fs::{self, File};
/// # use std::borrow::Borrow;
/// # use std::env::temp_dir;
/// # use std::io::Write;
/// # let root = temp_dir().join("bloguen-doctest").join("util-extract_actual_assets");
/// # fs::create_dir_all(&root).unwrap();
/// # File::create(root.join("image.png")).unwrap().write_all("image.png".as_bytes()).unwrap();
/// # /*
/// let root: PathBuf = /* obtained elsewhere */;
/// # */
/// let mut alloc = comrak::Arena::new();
///
/// let mut ast =
///     comrak::parse_document(&alloc, r#"[link](link.html)
///                                       ![img](image.png)"#, &MARKDOWN_OPTIONS);
/// assert_eq!(extract_actual_assets(&root, &mut ast), Ok(vec![&mut b"image.png".to_vec()]));
/// ```
pub fn extract_actual_assets<'a, P: AsRef<Path>>(post_source_dir: P, ast: &'a ComrakAstNode<'a>) -> Result<Vec<&'a mut Vec<u8>>, Error> {
    extract_actual_assets_impl(post_source_dir.as_ref(), ast)
}

fn extract_actual_assets_impl<'a>(post_source_dir: &Path, ast: &'a ComrakAstNode<'a>) -> Result<Vec<&'a mut Vec<u8>>, Error> {
    let mut out = vec![];

    for n in ast.descendants() {
        match n.data.borrow_mut().value {
            ComrakNodeValue::Link(ref mut link) |
            ComrakNodeValue::Image(ref mut link) => {
                {
                    let url = str::from_utf8(&link.url).map_err(|_| {
                            Error::Parse {
                                tp: "UTF-8 string",
                                wher: "URL list".into(),
                                more: None,
                            }
                        })?;

                    if !is_asset_link(url) {
                        continue;
                    }

                    let orig_path = concat_path(post_source_dir, url);
                    if !orig_path.exists() {
                        continue;
                    }
                }

                // The references are valid as long as the allocation arena is (i.e. 'a),
                // but there's only so much you can express :v
                out.push(unsafe {&mut *(&mut link.url as *mut Vec<u8>) as &'a mut Vec<u8>});
            }
            _ => {}
        }
    }

    Ok(out)
}

/// Check if the link points to a local relative asset.
///
/// # Examples
///
/// ```
/// # use bloguen::util::is_asset_link;
/// assert!(is_asset_link("assets/link.html"));
/// assert!(is_asset_link("assets/image.png"));
///
/// assert!(!is_asset_link("https://nabijaczleweli.xyz"));
/// ```
pub fn is_asset_link(link: &str) -> bool {
    Url::parse(link).is_err() && !link.starts_with('/')
}


/// Read the contents of the specified file into a `String`.
///
/// # Examples
///
/// Given the following:
///
/// ```plaintext
/// $ROOT
///   index.html
///   image.png
/// ```
///
/// The following holds:
///
/// ```
/// # use bloguen::util::read_file;
/// # use std::fs::{self, File};
/// # use std::path::PathBuf;
/// # use std::env::temp_dir;
/// # use std::io::Write;
/// # use bloguen::Error;
/// # let root = temp_dir().join("bloguen-doctest").join("ops-util-read_file");
/// # let _ = fs::remove_dir_all(&root);
/// # fs::create_dir_all(&root).unwrap();
/// # File::create(root.join("index.html")).unwrap().write_all("<html>{henlo}</html>".as_bytes()).unwrap();
/// # File::create(root.join("image.png")).unwrap()
/// #     .write_all(&[0xC3, 0x28, 0xA0, 0xA1, 0xE2, 0x28, 0xA1, 0xE2, 0x82, 0x28, 0xF0, 0x28,
/// #                  0x8C, 0xBC, 0xF0, 0x90, 0x28, 0xBC, 0xF0, 0x28, 0x8C, 0x28]).unwrap();
/// # /*
/// let root: PathBuf = /* obtained elsewhere */;
/// # */
///
/// assert_eq!(read_file(&("$ROOT/index.html".to_string(), root.join("index.html")), "root HTML"),
///            Ok("<html>{henlo}</html>".to_string()));
/// assert_eq!(read_file(&("$ROOT/image.png".to_string(), root.join("image.png")), "image"),
///            Err(Error::Parse {
///                tp: "UTF-8 string",
///                wher: "image".into(),
///                more: None,
///            }));
/// assert_eq!(read_file(&("$ROOT/nonexistant".to_string(), root.join("nonexistant")), "∅"),
///            Err(Error::FileNotFound {
///                who: "∅",
///                path: "$ROOT/nonexistant".into(),
///            }));
/// ```
pub fn read_file(whom: &(String, PathBuf), what_for: &'static str) -> Result<String, Error> {
    let mut buf = String::new();
    File::open(&whom.1).map_err(|e| if e.kind() == IoErrorKind::NotFound {
            Error::FileNotFound {
                who: what_for,
                path: whom.0.clone().into(),
            }
        } else {
            Error::Io {
                desc: what_for.into(),
                op: "open",
                more: Some(e.to_string().into()),
            }
        })?
        .read_to_string(&mut buf)
        .map_err(|_| {
            Error::Parse {
                tp: "UTF-8 string",
                wher: what_for.into(),
                more: None,
            }
        })?;
    Ok(buf)
}

/// Insert enough newlines at the start and end of the string to reach the specified count.
///
/// # Examples
///
/// ```
/// # use bloguen::util::newline_pad;
/// let mut data = "\nHenlo!\n".to_string();
/// newline_pad(&mut data, 0, 2);
/// assert_eq!(data, "\nHenlo!\n\n");
/// ```
pub fn newline_pad(val: &mut String, min_before: usize, min_after: usize) {
    let max = cmp::max(min_before, min_after);
    let mut cur_affix = String::with_capacity(max);

    let mut prefix_length = 0;
    let mut suffix_length = 0;
    for i in 1..=max {
        cur_affix.push('\n');

        if val.starts_with(&cur_affix) {
            prefix_length = i;
        }
        if val.ends_with(&cur_affix) {
            suffix_length = i;
        }
    }

    if prefix_length < min_before {
        val.insert_str(0, &cur_affix[..min_before - prefix_length]);
    }
    if suffix_length < min_after {
        val.push_str(&cur_affix[..min_after - suffix_length]);
    }
}

/// Parse a datetime specifier in the [`format_output()`](../ops/fn.format_output.html) argument style.
///
/// A couple presets are accepted:
/// * [RFC2822](https://docs.rs/chrono/0.4.6/chrono/struct.DateTime.html#method.to_rfc2822) –
///   `rfc2822`, `rfc_2822`, `RFC2822`, `RFC_2822`
/// * [RFC3339](https://docs.rs/chrono/0.4.6/chrono/struct.DateTime.html#method.to_rfc3339) –
///   `rfc3339`, `rfc_3339`, `RFC3339`, `RFC_3339`
///
/// The standard [`strftime()`](https://docs.rs/chrono/0.4.6/chrono/format/strftime/index.html#specifiers) syntax,
/// but wrapped in `"`s.
///
/// # Examples
///
/// ```
/// # extern crate bloguen;
/// # extern crate chrono;
/// # use chrono::format::{StrftimeItems, Fixed, Item};
/// # use bloguen::util::parse_date_format_specifier;
/// assert_eq!(parse_date_format_specifier("rfc_2822"),
///            Some(vec![Item::Fixed(Fixed::RFC2822)].into()));
/// assert_eq!(parse_date_format_specifier("RFC3339"),
///            Some(vec![Item::Fixed(Fixed::RFC3339)].into()));
///
/// assert_eq!(parse_date_format_specifier("\"%Y %B %d\""),
///            Some(StrftimeItems::new("%Y %B %d").collect()));
///
/// assert!(parse_date_format_specifier("epoch").is_none());
/// ```
pub fn parse_date_format_specifier(spec: &str) -> Option<Cow<[TimeFormatItem]>> {
    static RFC2822_ITEMS: &[TimeFormatItem] = &[TimeFormatItem::Fixed(FixedTimeFormatItem::RFC2822)];
    static RFC3339_ITEMS: &[TimeFormatItem] = &[TimeFormatItem::Fixed(FixedTimeFormatItem::RFC3339)];

    match spec.trim() {
        "rfc2822" | "rfc_2822" | "RFC2822" | "RFC_2822" => Some(RFC2822_ITEMS.into()),
        "rfc3339" | "rfc_3339" | "RFC3339" | "RFC_3339" => Some(RFC3339_ITEMS.into()),
        s if s.starts_with('"') && s.ends_with('"') => Some(StrftimeFormatItems::new(&spec[1..spec.len() - 1]).collect()),
        _ => None,
    }
}

/// Normalise a `DateTime` with any offset to a `FixedOffset` one.
///
/// # Examples
///
/// ```
/// # extern crate bloguen;
/// # extern crate chrono;
/// # use bloguen::util::normalise_datetime;
/// # use chrono::{DateTime, TimeZone, Utc};
/// assert_eq!(normalise_datetime(&Utc.ymd(2018, 8, 27).and_hms(7, 21, 32)),
///            DateTime::parse_from_rfc3339("2018-08-27T07:21:32+00:00").unwrap());
/// ```
pub fn normalise_datetime<Tz: TimeZone>(whom: &DateTime<Tz>) -> DateTime<FixedOffset> {
    whom.with_timezone(&whom.offset().fix())
}

/// Trivially parse a standard funxion invocation notation.
///
/// Return value is `Some((name, arguments))`, all trimmed, if a funxion is found, `None` otherwise.
///
/// Stolen and adapted from
/// [`controller-display`](https://github.com/nabijaczleweli/controller-display/blob/d7abaa206/src/util/parse.cpp#L98-L115).
///
/// # Examples
///
/// ```
/// # use bloguen::util::parse_function_notation;
/// assert_eq!(parse_function_notation("post_date(rfc_2822)"),
///            Some(("post_date", vec!["rfc_2822"])));
/// assert_eq!(parse_function_notation("date(post, \"%Y %B %d\")"),
///            Some(("date", vec!["post", "\"%Y %B %d\""])));
/// assert_eq!(parse_function_notation("date()"),
///            Some(("date", vec![])));
///
/// assert!(parse_function_notation("(post)").is_none());
/// ```
pub fn parse_function_notation(mut from: &str) -> Option<(&str, Vec<&str>)> {
    match (from.find('('), from.find(')')) {
        (None, _) | (Some(0), _) => None,
        (Some(lparen), Some(rparen)) if rparen > lparen => {
            from = &from[0..rparen];
            let args = from[lparen + 1..].split(',').map(str::trim).collect();

            Some((from[0..lparen].trim(), if args == &[""] { vec![] } else { args }))
        }
        (Some(lparen), _) => Some((&from[0..lparen], vec![])),
    }
}

/// Correctly append the specified string onto the specified path.
///
/// Works well even for `"\\?\"` paths on Windows, which don't handle `".."`, e.g., well.
///
/// Only really required for user input.
///
/// # Examples
///
/// ```
/// # use bloguen::util::concat_path;
/// # use std::path::Path;
/// assert_eq!(concat_path("hi/my/name/is", "what"),
///            Path::new("hi/my/name/is/what"));
/// assert_eq!(concat_path("/hi/my/name/is/slim/shady", ".././../who"),
///            Path::new("/hi/my/name/is/who"));
/// ```
pub fn concat_path<W: Into<PathBuf>>(whom: W, with: &str) -> PathBuf {
    concat_path_impl(whom.into(), with)
}

fn concat_path_impl(mut whom: PathBuf, with: &str) -> PathBuf {
    for seg in with.split(path::is_separator) {
        match seg {
            "" | "." => {}
            ".." => {
                whom.pop();
            }
            _ => whom.push(seg),
        }
    }

    whom
}

/// Try to get the default language for the system/user/environment.
///
/// On Windows, checks `GetLocaleInfoEx()`.
///
/// On non-Windows, checks `$LANG`, then `$LANGUAGE`, then `$LC_NAME`.
///
/// # Examples
///
/// ```no_run
/// # use bloguen::util::default_language;
/// // On Linux, if `LANG=en_GB.utf8`.
/// assert_eq!(default_language(), Some("en-GB".to_string()));
///
/// // On Windows, if language is set to Polish.
/// assert_eq!(default_language(), Some("pl".to_string()));
///
/// // If the language cannot be detected:
/// assert_eq!(default_language(), None);
/// ```
pub fn default_language() -> Option<String> {
    default_language_impl()
}

/// Try to get the name of the currently logged-in user.
///
/// On Windows, checks `GetUserName()`.
///
/// On non-Windows, checks `getlogin_r(3)`, then `$USER`.
pub fn current_username() -> Option<String> {
    current_username_impl()
}
