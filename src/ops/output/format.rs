use self::super::super::super::util::{parse_date_format_specifier, parse_function_notation};
use chrono::{FixedOffset, DateTime, TimeZone, Offset, Local, Utc};
use self::super::super::{WrappedElement, LanguageTag};
use self::super::super::super::Error;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::borrow::Cow;
use std::io::Write;


/// Fill out an HTML template.
///
/// All fields must be addressed even if formatted to be empty.
///
/// # Examples
///
/// ```
/// # extern crate bloguen;
/// # extern crate chrono;
/// # use bloguen::ops::{ScriptElement, StyleElement, format_output};
/// # use bloguen::util::LANGUAGE_EN_GB;
/// # use chrono::DateTime;
/// let head = r###"<!--
/// nabijaczleweli.xyz (c) by nabijaczleweli@gmail.com (nabijaczleweli)
/// ​
/// nabijaczleweli.xyz is licensed under a
/// Creative Commons Attribution 4.0 International License.
/// ​
/// You should have received a copy of the license along with this
/// work. If not, see <https://creativecommons.org/licenses/by/4.0/>.
/// -->
///
///
/// <!-- RSS_PUB_DATE: "{date(post, rfc_2822)}" -->
/// <!DOCTYPE html>
/// <html lang="{language}">
/// <head>
///     <meta charset="utf-8">
///     <meta http-equiv="X-UA-Compatible" content="IE=Edge">
///     <meta name="viewport" content="width=device-width,initial-scale=1">
///     <meta name="author" content="{author}">
///     <meta name="description" content="{data-desc}">
///     <title>{title} – generated { date ( now_utc, rfc3339 ) }</title>
///
///     <link href="/kaschism/assets/column.css" rel="stylesheet" />
///     <link href="../Roboto-font.css" rel="stylesheet" />
///     <link href="../the_taste_of_mi/Merriweather-font.css" rel="stylesheet" />
///     <link href="/content/assets/common.css" rel="stylesheet" />
///     {styles}
///     {scripts}
/// </head>
///     <body>
/// "###;
/// let global_data = vec![].into_iter().collect();
/// let local_data = vec![("desc".to_string(), "Każdy koniec to nowy początek [PL]".to_string())].into_iter().collect();
/// let mut out = vec![];
/// assert_eq!(
///     format_output(head, "Блогг", &LANGUAGE_EN_GB, &global_data, &local_data,
///                   "release-front - a generic release front-end, like Patchwork's", "nabijaczleweli",
///                   &DateTime::parse_from_rfc3339("2018-09-06T18:32:22+02:00").unwrap(),
///                   &[&[StyleElement::from_link("//nabijaczleweli.xyz/kaschism/assets/column.css")],
///                     &[StyleElement::from_literal(".indented { text-indent: 1em; }")]],
///                   &[&[ScriptElement::from_link("/content/assets/syllable.js")],
///                     &[ScriptElement::from_literal("document.getElementById(\"title\").innerText = \"Наган\";")]],
///                   &mut out, "test blog").unwrap(),
///     "test blog");
/// println!("{}", String::from_utf8(out).unwrap());
/// panic!();
/// ```
pub fn format_output<W, E, Tz, St, Sc>(to_format: &str, blog_name: &str, language: &LanguageTag, global_data: &BTreeMap<String, String>,
                                       local_data: &BTreeMap<String, String>, title: &str, author: &str, post_date: &DateTime<Tz>, styles: &[&[St]],
                                       scripts: &[&[Sc]], into: &mut W, out_name_err: E)
                                       -> Result<Cow<'static, str>, Error>
    where W: Write,
          E: Into<Cow<'static, str>>,
          Tz: TimeZone,
          St: WrappedElement,
          Sc: WrappedElement
{
    format_output_impl(to_format,
                       blog_name,
                       language,
                       global_data,
                       local_data,
                       title,
                       author,
                       normalise_datetime(post_date),
                       styles,
                       scripts,
                       into,
                       out_name_err.into())
}

pub fn format_output_impl<W, St, Sc>(mut to_format: &str, blog_name: &str, language: &LanguageTag, global_data: &BTreeMap<String, String>,
                                     local_data: &BTreeMap<String, String>, title: &str, author: &str, post_date: DateTime<FixedOffset>, styles: &[&[St]],
                                     scripts: &[&[Sc]], into: &mut W, out_name_err: Cow<'static, str>)
                                     -> Result<Cow<'static, str>, Error>
    where W: Write,
          St: WrappedElement,
          Sc: WrappedElement
{
    let mut out_name_err = Some(out_name_err);

    let mut byte_pos = 0usize;
    while let Some(idx) = to_format.find(|ref c| ['{', '}'].contains(c)) {
        let (before, mut after) = to_format.split_at(idx);

        into.write_all(before.as_bytes()).map_err(|e| err_io("write", format!("{} when writing unformatted output", e), out_name_err.take().unwrap()))?;
        byte_pos += before.len();

        if after.starts_with("{{") {
            into.write_all(&['{' as u8]).map_err(|e| err_io("write", format!("{} when writing escaped opening curly brace", e), out_name_err.take().unwrap()))?;
            byte_pos += 2;
            after = &after[2..];
        } else if after.starts_with("}}") {
            into.write_all(&['}' as u8]).map_err(|e| err_io("write", format!("{} when writing escaped closing curly brace", e), out_name_err.take().unwrap()))?;
            byte_pos += 2;
            after = &after[2..];
        } else if after.starts_with("}") {
            return Err(err_parse(format!("stray closing brace at position {}", byte_pos), out_name_err.take().unwrap()));
        } else {
            // Must start with { – begin format sequence

            if let Some(idx) = after.find('}') {
                let (mut format_str, post) = after.split_at(idx);
                after = &post[1..]; // drop closing paren
                byte_pos += format_str.len() + 1; // plus closing paren

                format_str = &format_str[1..].trim(); // drop open paren

                match format_str {
                        "language" => into.write_all(language.as_bytes()).map_err(|e| (e, "language tag".into())),
                        "author" => into.write_all(author.as_bytes()).map_err(|e| (e, "author tag".into())),
                        "title" => into.write_all(title.as_bytes()).map_err(|e| (e, "title tag".into())),
                        "blog_name" => into.write_all(blog_name.as_bytes()).map_err(|e| (e, "blog_name tag".into())),

                        "styles" => {
                            Result::from_iter(styles.iter().map(|ss| {
                                Result::from_iter(ss.iter().map(|s| {
                                    into.write_all(s.head_b()).map_err(|e| (e, "style tag header".into()))?;
                                    into.write_all(s.content_b()).map_err(|e| (e, "style tag footer".into()))?;
                                    into.write_all(s.foot_b()).map_err(|e| (e, "style tag footer".into()))?;

                                    Ok(())
                                }))
                            }))
                        }

                        "scripts" => {
                            Result::from_iter(scripts.iter().map(|ss| {
                                Result::from_iter(ss.iter().map(|s| {
                                    into.write_all(s.head_b()).map_err(|e| (e, "script tag header".into()))?;
                                    into.write_all(s.content_b()).map_err(|e| (e, "script tag footer".into()))?;
                                    into.write_all(s.foot_b()).map_err(|e| (e, "script tag footer".into()))?;

                                    Ok(())
                                }))
                            }))
                        }

                        key if key.starts_with("data-") => {
                            let key = &key["data-".len()..];
                            match local_data.get(key).or_else(|| global_data.get(key)) {
                                Some(data) => into.write_all(data.as_bytes()).map_err(|e| (e, format!("data-{} tag with value {}", key, data).into())),
                                None => return Err(err_parse(format!("missing value for data-{}", key), out_name_err.take().unwrap())),
                            }
                        }

                        _ => {
                            match parse_function_notation(format_str) {
                                Some(("date", args)) => {
                                    if args.len() != 2 {
                                        return Err(err_parse(format!("{} is an invalid amount of arguments to two-argument `date(of_what, format)` \
                                                                      function, around position {}",
                                                                     args.len(),
                                                                     byte_pos),
                                                             out_name_err.take().unwrap()));
                                    }

                                    let date_format = parse_date_format_specifier(args[1]).ok_or_else(|| {
                                            err_parse(format!("invalid date format specifier {} around position {}", args[1], byte_pos),
                                                      out_name_err.take().unwrap())
                                        })?;
                                    let date = match args[0] {
                                        "post" => Cow::Borrowed(&post_date),

                                        "now_utc" => Cow::Owned(normalise_datetime(&Utc::now())),
                                        "now_local" => Cow::Owned(normalise_datetime(&Local::now())),

                                        of_what => {
                                            return Err(err_parse(format!("{} is an unrecognised date specifier (accepted: post, now_{{utc,local}}), around \
                                                                          position {}",
                                                                         of_what,
                                                                         byte_pos),
                                                                 out_name_err.take().unwrap()))
                                        }
                                    };

                                    into.write_fmt(format_args!("{}", date.format_with_items(date_format.to_vec().into_iter())))
                                        .map_err(|e| (e, format!("{} date as {}", args[0], args[1]).into()))
                                }

                                Some((fname, args)) => {
                                    return Err(err_parse(format!("unrecognised format function {} with arguments {:?} at position {}", fname, args, byte_pos),
                                                         out_name_err.take().unwrap()))
                                }
                                _ => {
                                    return Err(err_parse(format!("unrecognised format specifier {} at position {}", format_str, byte_pos),
                                                         out_name_err.take().unwrap()))
                                }
                            }
                        }
                    }.map_err(|(e, d): (_, Cow<'static, str>)| err_io("write", format!("{} when writing substituted {}", e, d), out_name_err.take().unwrap()))?;
            } else {
                return Err(err_parse(format!("unmatched open brace at position {}", byte_pos), out_name_err.take().unwrap()));
            }
        }

        to_format = after;
    }

    into.write_all(to_format.as_bytes()).map_err(|e| err_io("write", format!("{} when writing unformatted output", e), out_name_err.take().unwrap()))?;

    Ok(out_name_err.unwrap())
}

fn normalise_datetime<Tz: TimeZone>(whom: &DateTime<Tz>) -> DateTime<FixedOffset> {
    whom.with_timezone(&whom.offset().fix())
}

fn err_io<M: Into<Cow<'static, str>>>(op: &'static str, more: M, out_name_err: Cow<'static, str>) -> Error {
    err_io_impl(op, more.into(), out_name_err)
}

fn err_parse<M: Into<Cow<'static, str>>>(more: M, out_name_err: Cow<'static, str>) -> Error {
    err_parse_impl(more.into(), out_name_err)
}

fn err_io_impl(op: &'static str, more: Cow<'static, str>, out_name_err: Cow<'static, str>) -> Error {
    Error::Io {
        desc: out_name_err,
        op: op,
        more: Some(more),
    }
}

fn err_parse_impl(more: Cow<'static, str>, out_name_err: Cow<'static, str>) -> Error {
    Error::Parse {
        tp: "unformatted input",
        wher: out_name_err,
        more: Some(more),
    }
}
