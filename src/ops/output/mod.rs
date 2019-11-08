mod machine_readable;
mod paragraph_passer;
mod wrapped_element;
mod format;
mod feed;

use std::borrow::Cow;
use self::super::super::Error;

pub use self::feed::{feed_type_post_footer, feed_atom_post_footer, feed_rss_post_footer, feed_type_post_header, feed_atom_post_header, feed_rss_post_header,
                     feed_type_post_body, feed_atom_post_body, feed_rss_post_body, feed_type_footer, feed_type_header, feed_atom_footer, feed_rss_footer,
                     feed_atom_header, feed_rss_header};
pub use self::wrapped_element::{WrappedElementImpl, WrappedElement, ScriptElement, StyleElement};
pub use self::machine_readable::{machine_output_json, machine_output_kind};
pub use self::paragraph_passer::ParagraphPasser;
pub use self::format::format_output;


fn err_io<M: Into<Cow<'static, str>>>(op: &'static str, more: M, out_name_err: Cow<'static, str>) -> Error {
    err_io_impl(op, more.into(), out_name_err)
}

fn err_io_impl(op: &'static str, more: Cow<'static, str>, out_name_err: Cow<'static, str>) -> Error {
    Error::Io {
        desc: out_name_err,
        op: op,
        more: more,
    }
}
