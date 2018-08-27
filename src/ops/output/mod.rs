mod machine_readable;
mod wrapped_element;
mod format;

use std::borrow::Cow;
use self::super::super::Error;

pub use self::wrapped_element::{WrappedElement, ScriptElement, StyleElement};
pub use self::machine_readable::machine_output_json;
pub use self::format::format_output;


fn err_io<M: Into<Cow<'static, str>>>(op: &'static str, more: M, out_name_err: Cow<'static, str>) -> Error {
    err_io_impl(op, more.into(), out_name_err)
}

fn err_io_impl(op: &'static str, more: Cow<'static, str>, out_name_err: Cow<'static, str>) -> Error {
    Error::Io {
        desc: out_name_err,
        op: op,
        more: Some(more),
    }
}
