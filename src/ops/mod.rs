mod language_tag;
mod descriptor;
mod metadata;
mod tag_name;
mod output;
mod post;

pub use self::output::{ParagraphPasser, WrappedElement, ScriptElement, StyleElement, machine_output_json, machine_output_kind, feed_type_header,
                       feed_rss_header, format_output};
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
