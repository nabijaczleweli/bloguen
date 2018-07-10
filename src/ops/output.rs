use std::collections::BTreeMap;
use self::super::super::Error;
use self::super::LanguageTag;
use std::borrow::Cow;
use std::io::Write;


/// Fill out an HTML template.
///
/// All fields must be addressed even if formatted to be empty.
///
/// # Examples
///
/// ```
/// # use bloguen::util::LANGUAGE_EN_GB;
/// # use bloguen::ops::format_output;
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
/// <!-- RSS_PUB_DATE: "{post_date(rfc_2822)}" -->
/// <!DOCTYPE html>
/// <html lang="{language}">
/// <head>
///     <meta charset="utf-8">
///     <meta http-equiv="X-UA-Compatible" content="IE=Edge">
///     <meta name="viewport" content="width=device-width,initial-scale=1">
///     <meta name="author" content="{author}">
///     <meta name="description" content="{data-desc}">
///     <title>{title}</title>
///
///     <link href="/kaschism/assets/column.css" rel="stylesheet" />
///     <link href="../Roboto-font.css" rel="stylesheet" />
///     <link href="../the_taste_of_mi/Merriweather-font.css" rel="stylesheet" />
///     <link href="/content/assets/common.css" rel="stylesheet" />
///     <style type="text/css">
///         {style}
///     </style>
///     {style_links}
///     {script_links}
/// </head>
///     <body>
/// "###;
/// let global_data = vec![].into_iter().collect();
/// let local_data = vec![("desc".to_string(), "Każdy koniec to nowy początek [PL]".to_string())].into_iter().collect();
/// let mut out = vec![];
/// format_output(head, &LANGUAGE_EN_GB, &global_data, &local_data, &mut out).unwrap();
/// println!("{}", String::from_utf8(out).unwrap());
/// panic!();
/// ```
pub fn format_output<W: Write>(mut to_format: &str, language: &LanguageTag, global_data: &BTreeMap<String, String>, local_data: &BTreeMap<String, String>,
                               into: &mut W)
                               -> Result<(), Error> {
    let mut byte_pos = 0usize;
    while let Some(idx) = to_format.find(|ref c| ['{', '}'].contains(c)) {
        let (before, mut after) = to_format.split_at(idx);

        into.write_all(before.as_bytes()).map_err(|e| err_io("write", format!("{} when writing unformatted output", e)))?;
        byte_pos += before.len();

        if after.starts_with("{{") {
            into.write_all(&['{' as u8]).map_err(|e| err_io("write", format!("{} when writing escaped opening curly brace", e)))?;
            byte_pos += 2;
        } else if after.starts_with("}}") {
            into.write_all(&['}' as u8]).map_err(|e| err_io("write", format!("{} when writing escaped closing curly brace", e)))?;
            byte_pos += 2;
        } else if after.starts_with("}") {
            return Err(err_parse(format!("stray closing brace at position {}", byte_pos)));
        } else {
            // Must start with { – begin format sequence

            if let Some(idx) = after.find('}') {
                let (mut format_str, post) = after.split_at(idx);
                after = &post[1..]; // drop closing paren
                byte_pos += format_str.len() + 1; // plus closing paren

                format_str = &format_str[1..].trim(); // drop open paren

                match format_str {
                        "language" => into.write_all(language.as_bytes()).map_err(|e| (e, "language tag".into())),
                        key if key.starts_with("data-") => {
                            let key = &key["data-".len()..];
                            match local_data.get(key).or_else(|| global_data.get(key)) {
                                Some(data) => into.write_all(data.as_bytes()).map_err(|e| (e, format!("data-{} tag with value {}", key, data).into())),
                                None => return Err(err_parse(format!("missing value for data-{}", key))),
                            }
                        }
                        _ => return Err(err_parse(format!("unrecognised format specifier {} at position {}", format_str, byte_pos))),
                    }.map_err(|(e, d): (_, Cow<'static, str>)| err_io("write", format!("{} when writing substituted {}", e, d)))?;
            } else {
                return Err(err_parse(format!("unmatched open brace at position {}", byte_pos)));
            }
        }

        to_format = after;
    }

    Ok(())

    // ASSETS.iter().fold(format_strings.iter().enumerate().fold(to_format.to_string(), |d, (i, s)| d.replace(&format!("{{{}}}",
    // i), s.as_ref())),
    //                   |d, (k, v)| d.replace(&format!("{{{}}}", k), v))
}

fn err_io<M: Into<Cow<'static, str>>>(op: &'static str, more: M) -> Error {
    err_io_impl(op, more.into())
}

fn err_parse<M: Into<Cow<'static, str>>>(more: M) -> Error {
    err_parse_impl(more.into())
}

fn err_io_impl(op: &'static str, more: Cow<'static, str>) -> Error {
    Error::Io {
        desc: "formatted output", // TODO
        op: op,
        more: Some(more),
    }
}

fn err_parse_impl(more: Cow<'static, str>) -> Error {
    Error::Parse {
        tp: "unformatted input", // TODO
        wher: "formatted output",
        more: Some(more),
    }
}
