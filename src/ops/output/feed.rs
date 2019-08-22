use self::super::super::super::util::{XmlEscapeWrite, normalise_datetime, BLOGUEN_VERSION};
use chrono::format::{Fixed as FixedTimeFormatItem, Item as TimeFormatItem};
use chrono::{FixedOffset, DateTime, TimeZone, Local};
use self::super::super::{LanguageTag, FeedType};
use std::io::{Error as IoError, Write};
use self::super::super::super::Error;
use self::super::err_io;
use std::fmt::Display;
use std::borrow::Cow;


static FEED_RSS_HEAD: &'static str = include_str!("../../../assets/element_wrappers/feed/rss.head");
static FEED_RSS_FOOT: &'static str = include_str!("../../../assets/element_wrappers/feed/rss.foot");


/// Get the correct feed output funxion for the specified feed kind.
///
/// Returns [`feed_rss_header()`](fn.feed_rss_header.html) for `FeedType::Rss`.
pub fn feed_type_header<W, E>(kind: &FeedType)
                              -> (fn(blog_name: &str,
                                     language: &LanguageTag,
                                     author: &str,
                                     link: Option<Cow<'static, str>>,
                                     into: &mut W,
                                     out_name_err: E)
                                     -> Result<Cow<'static, str>, Error>)
    where W: Write,
          E: Into<Cow<'static, str>>
{
    match kind {
        FeedType::Rss => feed_rss_header,
        FeedType::Atom => unimplemented!(),
    }
}

/// Get the correct feed output funxion for the specified feed kind.
///
/// Returns [`feed_rss_footer()`](fn.feed_rss_footer.html) for `FeedType::Rss`.
pub fn feed_type_footer<W, E>(kind: &FeedType) -> (fn(into: &mut W, out_name_err: E) -> Result<Cow<'static, str>, Error>)
    where W: Write,
          E: Into<Cow<'static, str>>
{
    match kind {
        FeedType::Rss => feed_rss_footer,
        FeedType::Atom => unimplemented!(),
    }
}

/// Get the correct feed output funxion for the specified feed kind.
///
/// Returns [`feed_rss_post_header()`](fn.feed_rss_post_header.html) for `FeedType::Rss`.
pub fn feed_type_post_header<W, E, Tz>(kind: &FeedType)
                                       -> (fn(post_name: &str,
                                              post_id_name: &str,
                                              author: &str,
                                              link: &str,
                                              post_date: &DateTime<Tz>,
                                              into: &mut W,
                                              out_name_err: E)
                                              -> Result<Cow<'static, str>, Error>)
    where Tz: TimeZone,
          W: Write,
          E: Into<Cow<'static, str>>
{
    match kind {
        FeedType::Rss => feed_rss_post_header,
        FeedType::Atom => unimplemented!(),
    }
}

/// Get the correct feed output writer for the specified feed kind.
///
/// Returns [`feed_rss_post_body()`](fn.feed_rss_post_body.html) for `FeedType::Rss`.
pub fn feed_type_post_body<'w, W>(kind: &FeedType) -> (fn(into: &'w mut W) -> Box<dyn Write + 'w>)
    where W: Write
{
    match kind {
        FeedType::Rss => feed_rss_post_body,
        FeedType::Atom => unimplemented!(),
    }
}

/// Get the correct feed output funxion for the specified feed kind.
///
/// Returns [`feed_rss_post_footer()`](fn.feed_rss_post_footer.html) for `FeedType::Rss`.
pub fn feed_type_post_footer<W, E>(kind: &FeedType) -> (fn(into: &mut W, out_name_err: E) -> Result<Cow<'static, str>, Error>)
    where W: Write,
          E: Into<Cow<'static, str>>
{
    match kind {
        FeedType::Rss => feed_rss_post_footer,
        FeedType::Atom => unimplemented!(),
    }
}


pub fn feed_rss_header<W, E>(blog_name: &str, language: &LanguageTag, author: &str, link: Option<Cow<'static, str>>, into: &mut W, out_name_err: E)
                             -> Result<Cow<'static, str>, Error>
    where W: Write,
          E: Into<Cow<'static, str>>
{
    feed_rss_header_impl(blog_name, language, author, link, into, out_name_err.into())
}

pub fn feed_rss_footer<W, E>(into: &mut W, out_name_err: E) -> Result<Cow<'static, str>, Error>
    where W: Write,
          E: Into<Cow<'static, str>>
{
    feed_rss_footer_impl(into, out_name_err.into())
}

pub fn feed_rss_post_header<W, E, Tz>(post_name: &str, post_id_name: &str, author: &str, link: &str, post_date: &DateTime<Tz>, into: &mut W, out_name_err: E)
                                      -> Result<Cow<'static, str>, Error>
    where Tz: TimeZone,
          W: Write,
          E: Into<Cow<'static, str>>
{
    feed_rss_post_header_impl(post_name, post_id_name, author, link, normalise_datetime(post_date), into, out_name_err.into())
}

pub fn feed_rss_post_footer<W, E>(into: &mut W, out_name_err: E) -> Result<Cow<'static, str>, Error>
    where W: Write,
          E: Into<Cow<'static, str>>
{
    feed_rss_post_footer_impl(into, out_name_err.into())
}

fn feed_rss_header_impl<W>(blog_name: &str, language: &LanguageTag, author: &str, link: Option<Cow<'static, str>>, into: &mut W,
                           out_name_err: Cow<'static, str>)
                           -> Result<Cow<'static, str>, Error>
    where W: Write
{
    let mut out_name_err = Some(out_name_err);

    (|| {
            into.write_all(FEED_RSS_HEAD.as_bytes()).map_err(|e| (e, "header".into()))?;

            write_tag("title", blog_name, into)?;
            write_tag("author", author, into)?;
            if let Some(link) = link {
                write_tag("link", link, into)?;
            }
            write_tag("description", blog_name, into)?;
            write_tag("language", language, into)?;
            write_tag("generator", format!("bloguen {}", BLOGUEN_VERSION), into)?;

            let now_local = normalise_datetime(&Local::now());
            write_date("pubDate", &now_local, FixedTimeFormatItem::RFC2822, into)?;
            write_date("lastBuildDate", &now_local, FixedTimeFormatItem::RFC2822, into)?;

            Ok(())
        })().map_err(|(e, d): (_, Cow<'static, str>)| err_io("write", format!("{} when writing RSS feed output {}", e, d), out_name_err.take().unwrap()))?;

    Ok(out_name_err.unwrap())
}

fn feed_rss_footer_impl<W>(into: &mut W, out_name_err: Cow<'static, str>) -> Result<Cow<'static, str>, Error>
    where W: Write
{
    let mut out_name_err = Some(out_name_err);

    (|| {
            into.write_all(FEED_RSS_FOOT.as_bytes()).map_err(|e| (e, "footer".into()))?;

            Ok(())
        })().map_err(|(e, d): (_, Cow<'static, str>)| err_io("write", format!("{} when writing RSS feed output {}", e, d), out_name_err.take().unwrap()))?;

    Ok(out_name_err.unwrap())
}

fn feed_rss_post_header_impl<W>(post_name: &str, post_id_name: &str, author: &str, link: &str, post_date: DateTime<FixedOffset>, into: &mut W,
                                out_name_err: Cow<'static, str>)
                                -> Result<Cow<'static, str>, Error>
    where W: Write
{
    let mut out_name_err = Some(out_name_err);

    (|| {
            into.write_all(b"\n").map_err(|e| (e, "header separator".into()))?;
            into.write_all(b"    <item>\n").map_err(|e| (e, "header item tag".into()))?;

            write_tag_post("title", post_name, into)?;
            write_tag_post("author", author, into)?;
            write_tag_post("link", link, into)?;
            write_date_post("pubDate", &post_date, FixedTimeFormatItem::RFC2822, into)?;
            write_tag_post("guid", post_id_name, into)?;

            into.write_all(b"      <description>\n").map_err(|e| (e, "header description tag".into()))?;

            Ok(())
        })().map_err(|(e, d): (_, Cow<'static, str>)| err_io("write", format!("{} when writing RSS feed post output {}", e, d), out_name_err.take().unwrap()))?;

    Ok(out_name_err.unwrap())
}

pub fn feed_rss_post_body<'w, W>(into: &'w mut W) -> Box<dyn Write + 'w>
    where W: Write
{
    Box::new(XmlEscapeWrite(into))
}

fn feed_rss_post_footer_impl<W>(into: &mut W, out_name_err: Cow<'static, str>) -> Result<Cow<'static, str>, Error>
    where W: Write
{
    let mut out_name_err = Some(out_name_err);

    (|| {
            into.write_all(b"      </description>\n").map_err(|e| (e, "footer description tag".into()))?;
            into.write_all(b"    </item>\n").map_err(|e| (e, "footer item tag".into()))?;

            Ok(())
        })().map_err(|(e, d): (_, Cow<'static, str>)| err_io("write", format!("{} when writing RSS feed post output {}", e, d), out_name_err.take().unwrap()))?;

    Ok(out_name_err.unwrap())
}

fn write_date<W: Write>(name: &str, value: &DateTime<FixedOffset>, format: FixedTimeFormatItem, into: &mut W) -> Result<(), (IoError, Cow<'static, str>)> {
    write_tag(name, value.format_with_items([TimeFormatItem::Fixed(format)].iter().cloned()), into)
}

fn write_date_post<W: Write>(name: &str, value: &DateTime<FixedOffset>, format: FixedTimeFormatItem, into: &mut W) -> Result<(), (IoError, Cow<'static, str>)> {
    write_tag_post(name, value.format_with_items([TimeFormatItem::Fixed(format)].iter().cloned()), into)
}

fn write_tag<W: Write, V: Display>(name: &str, value: V, into: &mut W) -> Result<(), (IoError, Cow<'static, str>)> {
    write_tag_indented(b"    <", name, value, into)
}

fn write_tag_post<W: Write, V: Display>(name: &str, value: V, into: &mut W) -> Result<(), (IoError, Cow<'static, str>)> {
    write_tag_indented(b"      <", name, value, into)
}

fn write_tag_indented<W: Write, V: Display>(indented: &[u8], name: &str, value: V, mut into: &mut W) -> Result<(), (IoError, Cow<'static, str>)> {
    into.write_all(indented).map_err(|e| (e, "tag pre start".into()))?;
    into.write_all(name.as_bytes()).map_err(|e| (e, format!("{} open tag name", name).into()))?;
    into.write_all(b">").map_err(|e| (e, "tag pre end".into()))?;
    XmlEscapeWrite(&mut into).write_fmt(format_args!("{}", value)).map_err(|e| (e, format!("{} tag content", name).into()))?;
    into.write_all(b"</").map_err(|e| (e, "tag post start".into()))?;
    into.write_all(name.as_bytes()).map_err(|e| (e, format!("{} closing tag name", name).into()))?;
    into.write_all(b">\n").map_err(|e| (e, "tag post end".into()))?;

    Ok(())
}
