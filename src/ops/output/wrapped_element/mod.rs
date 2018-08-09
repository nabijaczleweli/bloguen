mod style_element;
mod script_element;

pub use self::style_element::StyleElement;
pub use self::script_element::ScriptElement;


/// An element with a preface, a content, and a postface.
///
/// More efficient than concatting them together.
///
/// The `*_b()` versions are I/O helpers.
pub trait WrappedElement {
    /// Characters to put before the content.
    fn head(&self) -> &str;

    /// The content itself.
    fn content(&self) -> &str;

    /// Characters to put after the content.
    fn foot(&self) -> &str;

    /// Byte representation of pre-content.
    fn head_b(&self) -> &[u8] {
        self.head().as_bytes()
    }

    /// Byte representation of the content.
    fn content_b(&self) -> &[u8] {
        self.content().as_bytes()
    }

    /// Byte representation of post-content.
    fn foot_b(&self) -> &[u8] {
        self.foot().as_bytes()
    }
}
