mod style_element;

pub use self::style_element::StyleElement;


pub trait WrappedElement {
    fn head(&self) -> &str;
    fn content(&self) -> &str;
    fn foot(&self) -> &str;

    fn head_b(&self) -> &[u8] {
        self.head().as_bytes()
    }

    fn content_b(&self) -> &[u8] {
        self.content().as_bytes()
    }

    fn foot_b(&self) -> &[u8] {
        self.foot().as_bytes()
    }
}
