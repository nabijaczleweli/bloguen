//! Functions composable to generate blogues
//!
//! The following steps will yield something akin to
//! the [`bloguen` executable](https://rawcdn.githack.com/nabijaczleweli/bloguen/man/bloguen.1.html):
//!   1. [`BlogueDescriptor::read()`](struct.BlogueDescriptor.html#method.read) to read a blogue descriptor from the filesystem
//!   2. [`BloguePost::list()`](struct.BloguePost.html#method.list) and pipe the results into
//!      [`BloguePost::new()`](struct.BloguePost.html#method.new) to discover and load posts from the filesystem
//!   3. Read in the post header and footer, and, if applicable, index header, footer, and center
//!   4. [`{Script,Style}Element::load()`](struct.StyleElement.html#method.load) the blogue and index descriptors
//!   5. [`BlogueDescriptor::create_feed_output()`](struct.BlogueDescriptor.html#method.create_feed_output), yielding feed files
//!   6. [`BlogueDescriptor::generate_feed_head()`](struct.BlogueDescriptor.html#method.generate_feed_head)s
//!   7. For each discovered post:
//!     1. [`PostMetadata::read_or_default()`](struct.PostMetadata.html#method.read_or_default) to discover and load any
//!        metadata that might be present from the filesystem
//!     2. [`{Script,Style}Element::load()`](struct.ScriptElement.html#method.load)s
//!     3. [`TagName::load_additional_post_tags()`](struct.TagName.html#method.load_additional_post_tags)s to discover and load
//!        any additional tags that might be present from the filesystem
//!     4. For each pair in [`BlogueDescriptor::machine_data`](struct.BlogueDescriptor.html#structfield.machine_data):
//!        1. [`BloguePost::create_machine_output()`](struct.BloguePost.html#method.create_machine_output),
//!           yielding the machine data file
//!        2. [`BloguePost::generate_machine()`](struct.BloguePost.html#method.generate_machine) into the file from above
//!     5. If index file requested, [`BloguePost::generate_machine(MachineDataKind::Json)`](struct.BloguePost.html#method.generate_machine) into the script file
//!     6. [`BloguePost::generate_feed_head()`](struct.BloguePost.html#method.generate_feed_head) into the feed files
//!     7. [`BloguePost::generate()`](struct.BloguePost.html#method.generate) to create the post HTML, with
//!        [`feed_type_post_body()`](fn.feed_type_post_body.html)s connected to the alt stream and, if requested, the index center buffer,
//!        and get the asset list
//!     8. [`BloguePost::copy_asset()`](struct.BloguePost.html#method.copy_asset) the returned percent-decoded links
//!        if they're assets
//!     9. [`BloguePost::generate_feed_foot()`](struct.BloguePost.html#method.generate_feed_foot) into the feed files
//!   7. [`BlogueDescriptor::generate_feed_foot()`](struct.BlogueDescriptor.html#method.generate_feed_foot)s
//!   8. If index requested:
//!     1. Concatenate the JSON machine data into an additional script
//!     2. Create an `index.html` file
//!     3. [`format_output()`](fn.format_output.html) the index header with the above script
//!     4. Write out the previously saved centers
//!     5. [`format_output()`](fn.format_output.html) the index header with the above script
//!
//! Variables available in [`format_output()`](fn.format_output.html):
//!
//! | Name                          | Description                                                                | Example                                                       |
//! | ----                          | -----------                                                                | -------                                                       |
//! | `language`                    | passed-in language in BCP47 format                                         | en-GB                                                         |
//! | `number`                      | default-formatted passed-in number                                         | 14                                                            |
//! | `title`                       | passed-in title, unformatted                                               | release-front - a generic release front-end, like Patchwork's |
//! | `author`                      | passed-in author, unformatted                                              | nabijaczleweli                                                |
//! | `raw_post_name`               | passed-in post name as it appeared on the filesystem, unformatted          | 004. 2018-03-30 Stir plate                                    |
//! | `normalised_post_name`        | passed-in normalised post name, unformatted                                | 004. 2018-03-30 06-00-51 Stir plate                           |
//! | `blog_name`                   | passed-in blog name, unformatted                                           | Блогг                                                         |
//! | `bloguen-version`             | current version of `bloguen`                                               | v0.1.0                                                        |
//! | `tags`                        |                                                                            | `<span class="post-tag">maths</span>`…                        |
//! | `tags()`                      | all passed-in tags with the default class (`post-tag`)                     | `<span class="post-tag">maths</span>`…                        |
//! | `tags(class)`                 | all passed-in tags with the specified class. headers and footers           | `<span class="пост-таг">maths</span>`…                        |
//! | `styles`                      | all the passed-in styles with their headers and footers                    | `<style type="text/css">* {color: magenta;}</style>`…         |
//! | `scripts`                     | all the passed-in scripts with their headers and footers                   | `<script type="text/javascript">alert("hewwo")</script>`…     |
//! | `data-name`                   | passed-in data under the `name` key, unformatted                           | hewwo                                                         |
//! | `date(post, format)`          | post date formatted with [`parse_date_format_specifier(format)`]           | Thu,  6 Sep 2018 18:32:22 +0200                               |
//! | `date(now_utc, format)`       | current date in UTC formatted with [`parse_date_format_specifier(format)`] | Thu,  6 Sep 2018 18:32:22 +0200                               |
//! | `machine_data(kind)`          | machine data of the specified kind                                         | `{"number": 3, "language": "en-GB", …}`…                      |
//! | `pass_paragraphs(count, var)` | parse `var` and write its contents formatted through [`ParagraphPasser`]   | `{"number": 3, "language": "en-GB", …}`…                      |
//!
//! [`parse_date_format_specifier(format)`]: fn.parse_date_format_specifier.html
//! [`ParagraphPasser`]: struct.ParagraphPasser.html

mod language_tag;
mod descriptor;
mod metadata;
mod tag_name;
mod output;
mod post;

pub use self::output::{ParagraphPasser, WrappedElement, ScriptElement, StyleElement, feed_type_post_footer, feed_atom_post_footer, feed_rss_post_footer,
                       feed_type_post_header, feed_atom_post_header, feed_rss_post_header, feed_type_post_body, feed_atom_post_body, feed_rss_post_body,
                       machine_output_json, machine_output_kind, feed_type_header, feed_type_footer, feed_atom_header, feed_rss_header, feed_atom_footer,
                       feed_rss_footer, format_output};
pub use self::descriptor::{BlogueDescriptorIndex, BlogueDescriptor};
pub use self::machine_data::MachineDataKind;
pub use self::center_order::CenterOrder;
pub use self::language_tag::LanguageTag;
pub use self::metadata::PostMetadata;
pub use self::feed_type::FeedType;
pub use self::tag_name::TagName;
pub use self::post::BloguePost;


include!(concat!(env!("OUT_DIR"), "/simple-parsable/center_order.rs"));

impl Default for CenterOrder {
    fn default() -> CenterOrder {
        CenterOrder::Forward
    }
}


include!(concat!(env!("OUT_DIR"), "/simple-parsable/machine_data.rs"));

impl MachineDataKind {
    /// Get extension to use for saving this kind (without dot).
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::MachineDataKind;
    /// assert_eq!(MachineDataKind::Json.extension(), "json");
    /// ```
    pub fn extension(&self) -> &'static str {
        match self {
            MachineDataKind::Json => "json",
        }
    }
}


include!(concat!(env!("OUT_DIR"), "/simple-parsable/feed_type.rs"));
