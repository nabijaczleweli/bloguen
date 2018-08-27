use chrono::format::{Fixed as FixedTimeFormatItem, Item as TimeFormatItem};
use chrono::{FixedOffset, DateTime, TimeZone, Offset, Local, Utc};
use self::super::super::{WrappedElement, LanguageTag, TagName};
use self::super::super::super::util::BLOGUEN_VERSION;
use std::collections::{BTreeMap, BTreeSet};
use std::io::{Error as IoError, Write};
use self::super::super::super::Error;
use std::iter::FromIterator;
use self::super::err_io;
use std::borrow::Cow;


lazy_static! {
    static ref HEAD: &'static str = include_str!("../../../assets/machine_wrappers/json/head.json").trim();
    static ref FOOT: &'static str = include_str!("../../../assets/machine_wrappers/json/foot.json").trim();

    static ref NUMBER_PRE: &'static str = include_str!("../../../assets/machine_wrappers/json/number-pre.json").trim();
    static ref NUMBER_CENTER: &'static str = include_str!("../../../assets/machine_wrappers/json/number-center.json").trim();
    static ref NUMBER_POST: &'static str = include_str!("../../../assets/machine_wrappers/json/number-post.json").trim();

    static ref STRING_PRE: &'static str = include_str!("../../../assets/machine_wrappers/json/string-pre.json").trim();
    static ref STRING_CENTER: &'static str = include_str!("../../../assets/machine_wrappers/json/string-center.json").trim();
    static ref STRING_POST: &'static str = include_str!("../../../assets/machine_wrappers/json/string-post.json").trim();

    static ref MAP_PRE: &'static str = include_str!("../../../assets/machine_wrappers/json/map-pre.json").trim();
    static ref MAP_CENTER: &'static str = include_str!("../../../assets/machine_wrappers/json/map-center.json").trim();
    static ref MAP_POST: &'static str = include_str!("../../../assets/machine_wrappers/json/map-post.json").trim();

    static ref ARRAY_PRE: &'static str = include_str!("../../../assets/machine_wrappers/json/array-pre.json").trim();
    static ref ARRAY_CENTER: &'static str = include_str!("../../../assets/machine_wrappers/json/array-center.json").trim();
    static ref ARRAY_POST: &'static str = include_str!("../../../assets/machine_wrappers/json/array-post.json").trim();

    static ref ARRAY_STRING_POST: &'static str = include_str!("../../../assets/machine_wrappers/json/array-string-post.json").trim();
    static ref ARRAY_STRING_POST_LAST: &'static str = include_str!("../../../assets/machine_wrappers/json/array-string-post-last.json").trim();
}

static NEWLINE: &str = "\n    ";
static NEWLINE_INDENTED: &str = "\n        ";
static CENTER_VALUE_SEPARATOR: &str = " ";


/// Fill out an HTML template.
///
/// All fields must be addressed even if formatted to be empty.
///
/// Care should be taken to ensure the arguments to this funxion are as close as possible to the arguments to
/// [`format_output()`](fn.format_output.html)
///
/// # Examples
///
/// ```
/// # extern crate bloguen;
/// # extern crate chrono;
/// # use bloguen::ops::{ScriptElement, StyleElement, machine_output_json};
/// # use bloguen::util::LANGUAGE_EN_GB;
/// # use chrono::DateTime;
/// let global_data = vec![].into_iter().collect();
/// let local_data =
///     vec![("desc".to_string(),
///           "Każdy koniec to nowy początek [PL]".to_string())].into_iter().collect();
/// let mut out = vec![];
/// let res = machine_output_json(
///     "Блогг", &LANGUAGE_EN_GB, &[&global_data, &local_data],
///     "003. 2018-02-05 release-front - a generic release front-end, like Patchwork's",
///     3, "release-front - a generic release front-end, like Patchwork's", "nabijaczleweli",
///     &DateTime::parse_from_rfc3339("2018-09-06T18:32:22+02:00").unwrap(),
///     &[&["vodka".parse().unwrap(), "depression".parse().unwrap()][..],
///       &["коммунизм".parse().unwrap()][..]],
///     &[&[StyleElement::from_link("//nabijaczleweli.xyz/kaschism/assets/column.css")],
///       &[StyleElement::from_literal(".indented { text-indent: 1em; }")]],
///     &[&[ScriptElement::from_link("/content/assets/syllable.js")],
///       &[ScriptElement::from_literal("alert(\"You're the 1`000`000th visitor!\");")]],
///     &mut out, "test blog");
/// assert_eq!(res, Ok("test blog".into()));
///
/// assert_eq!(String::from_utf8(out).unwrap(), r###"{
///     "number": 3,
///     "language": "en-GB",
///     "title": "release-front - a generic release front-end, like Patchwork's",
///     "author": "nabijaczleweli",
///
///     "raw_post_name": "003. 2018-02-05 release-front - a generic release front-end, like Patchwork\'s",
///     "blog_name": "Блогг",
///
///     "post_date_rfc3339": "2018-09-06T18:32:22 +0200",
///     "post_date_rfc2822": "Thu,  6 Sep 2018 18:32:22 +0200",
///     "generation_date_utc_rfc3339": "2018-09-06T18:32:22 +0200",
///     "generation_date_utc_rfc2822": "Thu,  6 Sep 2018 18:32:22 +0200",
///     "generation_date_local_rfc3339": "2018-09-06T18:32:22 +0200",
///     "generation_date_local_rfc2822": "Thu,  6 Sep 2018 18:32:22 +0200",
///
///     "tags": [
///         "vodka",
///         "depression",
///         "коммунизм"
///     ],
///     "additional_data": {
///         "data": "Każdy koniec to nowy początek [PL]"
///     }
///
///     "styles": [
///         "//nabijaczleweli.xyz/kaschism/assets/column.css",
///         ".indented { text-indent: 1em; }"
///     ],
///     "scripts": [
///         "/content/assets/syllable.js",
///         "alert(\"You're the 1`000`000th visitor!\");"
///     ],
///
///     "bloguen-version": "0.1.0"
///
/// }"###);
/// ```
pub fn machine_output_json<W, E, Tz, St, Sc>(blog_name: &str, language: &LanguageTag, additional_data_sets: &[&BTreeMap<String, String>],
                                             raw_post_name: &str, number: usize, title: &str, author: &str, post_date: &DateTime<Tz>, tags: &[&[TagName]],
                                             styles: &[&[St]], scripts: &[&[Sc]], into: &mut W, out_name_err: E)
                                             -> Result<Cow<'static, str>, Error>
    where W: Write,
          E: Into<Cow<'static, str>>,
          Tz: TimeZone,
          St: WrappedElement,
          Sc: WrappedElement
{
    machine_output_json_impl(blog_name,
                             language,
                             additional_data_sets,
                             raw_post_name,
                             number,
                             title,
                             author,
                             normalise_datetime(post_date),
                             tags,
                             styles,
                             scripts,
                             into,
                             out_name_err.into())
}

fn machine_output_json_impl<W, St, Sc>(blog_name: &str, language: &LanguageTag, additional_data_sets: &[&BTreeMap<String, String>], raw_post_name: &str,
                                       number: usize, title: &str, author: &str, post_date: DateTime<FixedOffset>, tags: &[&[TagName]], styles: &[&[St]],
                                       scripts: &[&[Sc]], into: &mut W, out_name_err: Cow<'static, str>)
                                       -> Result<Cow<'static, str>, Error>
    where W: Write,
          St: WrappedElement,
          Sc: WrappedElement
{
    let mut out_name_err = Some(out_name_err);

    (|| {
            into.write_all(HEAD.as_bytes()).map_err(|e| (e, "header".into()))?;
            into.write_all(NEWLINE.as_bytes()).map_err(|e| (e, "newline".into()))?;

            into.write_all(NUMBER_PRE.as_bytes()).map_err(|e| (e, "number pre".into()))?;
            into.write_all("number".as_bytes()).map_err(|e| (e, "post number field name".into()))?;
            into.write_all(NUMBER_CENTER.as_bytes()).map_err(|e| (e, "number center".into()))?;
            into.write_all(CENTER_VALUE_SEPARATOR.as_bytes()).map_err(|e| (e, "number center".into()))?;
            into.write_fmt(format_args!("{}", number)).map_err(|e| (e, "post number".into()))?;
            into.write_all(NUMBER_POST.as_bytes()).map_err(|e| (e, "number post".into()))?;
            into.write_all(NEWLINE.as_bytes()).map_err(|e| (e, "newline".into()))?;

            write_string_variable("language", &language, into)?;
            write_string_variable("title", title, into)?;
            write_string_variable("author", author, into)?;
            into.write_all(NEWLINE.as_bytes()).map_err(|e| (e, "newline".into()))?;

            write_string_variable("raw_post_name", raw_post_name, into)?;
            write_string_variable("blog_name", blog_name, into)?;
            into.write_all(NEWLINE.as_bytes()).map_err(|e| (e, "newline".into()))?;

            write_date("post_date_rfc3339", &post_date, FixedTimeFormatItem::RFC3339, into)?;
            write_date("post_date_rfc2822", &post_date, FixedTimeFormatItem::RFC2822, into)?;

            let now_utc = normalise_datetime(&Utc::now());
            write_date("generation_date_utc_rfc3339", &now_utc, FixedTimeFormatItem::RFC3339, into)?;
            write_date("generation_date_utc_rfc2822", &now_utc, FixedTimeFormatItem::RFC2822, into)?;

            let now_local = normalise_datetime(&Local::now());
            write_date("generation_date_local_rfc3339", &now_local, FixedTimeFormatItem::RFC3339, into)?;
            write_date("generation_date_local_rfc2822", &now_local, FixedTimeFormatItem::RFC2822, into)?;
            into.write_all(NEWLINE.as_bytes()).map_err(|e| (e, "newline".into()))?;

            write_array("tags", tags, |t| &*t, into)?;
            write_data("additional_data", additional_data_sets, into)?;
            into.write_all(NEWLINE.as_bytes()).map_err(|e| (e, "newline".into()))?;

            write_array("styles", styles, |s| s.content(), into)?;
            write_array("scripts", scripts, |s| s.content(), into)?;
            into.write_all(NEWLINE.as_bytes()).map_err(|e| (e, "newline".into()))?;

            write_string_variable("bloguen-version", BLOGUEN_VERSION, into)?;

            into.write_all("\n".as_bytes()).map_err(|e| (e, "header".into()))?;
            into.write_all(FOOT.as_bytes()).map_err(|e| (e, "header".into()))?;

            Ok(())
        })().map_err(|(e, d): (_, Cow<'static, str>)| err_io("write", format!("{} when writing JSON machine output {}", e, d), out_name_err.take().unwrap()))?;

    Ok(out_name_err.unwrap())
}

fn write_date<W: Write>(name: &str, value: &DateTime<FixedOffset>, format: FixedTimeFormatItem, into: &mut W) -> Result<(), (IoError, Cow<'static, str>)> {
    into.write_all(STRING_PRE.as_bytes()).map_err(|e| (e, "string pre".into()))?;
    into.write_all(name.as_bytes()).map_err(|e| (e, format!("{} field name", name).into()))?;
    into.write_all(STRING_CENTER.as_bytes()).map_err(|e| (e, "string center".into()))?;
    into.write_all(CENTER_VALUE_SEPARATOR.as_bytes()).map_err(|e| (e, "string center".into()))?;
    into.write_fmt(format_args!("{}", value.format_with_items([TimeFormatItem::Fixed(format)].iter().cloned())))
        .map_err(|e| (e, format!("{} date", name).into()))?;
    into.write_all(STRING_POST.as_bytes()).map_err(|e| (e, "string post".into()))?;
    into.write_all(NEWLINE.as_bytes()).map_err(|e| (e, "newline".into()))?;

    Ok(())
}

fn write_string_variable<W: Write>(name: &str, value: &str, into: &mut W) -> Result<(), (IoError, Cow<'static, str>)> {
    into.write_all(STRING_PRE.as_bytes()).map_err(|e| (e, "string pre".into()))?;
    into.write_all(name.as_bytes()).map_err(|e| (e, format!("{} field name", name).into()))?;
    into.write_all(STRING_CENTER.as_bytes()).map_err(|e| (e, "string center".into()))?;
    into.write_all(CENTER_VALUE_SEPARATOR.as_bytes()).map_err(|e| (e, "string center".into()))?;
    into.write_fmt(format_args!("{:?}", value)).map_err(|e| (e, format!("{} field content", name).into()))?;
    into.write_all(STRING_POST.as_bytes()).map_err(|e| (e, "string post".into()))?;
    into.write_all(NEWLINE.as_bytes()).map_err(|e| (e, "newline".into()))?;

    Ok(())
}

fn write_array<El, M: Fn(&El) -> &str, W: Write>(name: &str, arrs: &[&[El]], map: M, into: &mut W) -> Result<(), (IoError, Cow<'static, str>)> {
    into.write_all(ARRAY_PRE.as_bytes()).map_err(|e| (e, "array pre".into()))?;
    into.write_all(name.as_bytes()).map_err(|e| (e, format!("{} field name", name).into()))?;
    into.write_all(ARRAY_CENTER.as_bytes()).map_err(|e| (e, "array center".into()))?;

    Result::from_iter(arrs.iter().enumerate().map(|(i, arr)| (i == arrs.len() - 1, arr)).map(|(ee, arr)| {
        Result::from_iter(arr.iter().enumerate().map(|(i, a)| (i == arr.len() - 1, a)).map(|(e, a)| {
            into.write_all(NEWLINE_INDENTED.as_bytes()).map_err(|e| (e, "indented newline".into()))?;

            into.write_fmt(format_args!("{:?}", map(a))).map_err(|e| (e, "string array element".into()))?;
            if !(e && ee) {
                into.write_all(ARRAY_STRING_POST.as_bytes()).map_err(|e| (e, "string array post".into()))?;
            } else {
                into.write_all(ARRAY_STRING_POST_LAST.as_bytes()).map_err(|e| (e, "string array post lasts".into()))?;
            }

            Ok(())
        }))
    }))?;

    into.write_all(NEWLINE.as_bytes()).map_err(|e| (e, "newline".into()))?;
    into.write_all(ARRAY_POST.as_bytes()).map_err(|e| (e, "array post".into()))?;
    into.write_all(NEWLINE.as_bytes()).map_err(|e| (e, "newline".into()))?;

    Ok(())
}

fn write_data<W: Write>(name: &str, datas: &[&BTreeMap<String, String>], into: &mut W) -> Result<(), (IoError, Cow<'static, str>)> {
    into.write_all(MAP_PRE.as_bytes()).map_err(|e| (e, "map pre".into()))?;
    into.write_all(name.as_bytes()).map_err(|e| (e, format!("{} field name", name).into()))?;
    into.write_all(MAP_CENTER.as_bytes()).map_err(|e| (e, "map center".into()))?;

    datas.iter()
        .rev()
        .flat_map(|dt| dt.iter())
        .fold(Ok(BTreeSet::new()), |mut acc, el| {
            if acc.is_err() || acc.as_ref().unwrap().contains(el.0) {
                return acc;
            }

            into.write_all(NEWLINE_INDENTED.as_bytes()).map_err(|e| (e, "indented newline".into()))?;

            into.write_all(STRING_PRE.as_bytes()).map_err(|e| (e, "aux data string map pre".into()))?;
            into.write_fmt(format_args!("{:?}", el.0)).map_err(|e| (e, "aux data string map element".into()))?;
            into.write_all(STRING_CENTER.as_bytes()).map_err(|e| (e, "aux data string map post".into()))?;
            into.write_fmt(format_args!("{:?}", el.1)).map_err(|e| (e, "aux data string map element".into()))?;
            into.write_all(STRING_POST.as_bytes()).map_err(|e| (e, "aux data string map post".into()))?;

            acc.as_mut().unwrap().insert(el.0);

            acc
        })?;

    into.write_all(NEWLINE.as_bytes()).map_err(|e| (e, "newline".into()))?;
    into.write_all(MAP_POST.as_bytes()).map_err(|e| (e, "map post".into()))?;

    Ok(())
}

fn normalise_datetime<Tz: TimeZone>(whom: &DateTime<Tz>) -> DateTime<FixedOffset> {
    whom.with_timezone(&whom.offset().fix())
}
