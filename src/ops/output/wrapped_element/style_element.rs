use self::super::super::super::super::util::read_file;
use self::super::super::super::super::Error;
use self::super::WrappedElement;
use std::path::PathBuf;
use std::borrow::Cow;
use serde::de;
use std::fmt;


lazy_static! {
    static ref STYLE_LINK_HEAD: &'static str = include_str!("../../../../assets/element_wrappers/style/link.head").trim();
    static ref STYLE_LINK_FOOT: &'static str = include_str!("../../../../assets/element_wrappers/style/link.foot").trim();

    static ref STYLE_LITERAL_HEAD: &'static str = include_str!("../../../../assets/element_wrappers/style/link.head").trim_left();
    static ref STYLE_LITERAL_FOOT: &'static str = include_str!("../../../../assets/element_wrappers/style/link.foot").trim_right();
}


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum StyleElementClass {
    Link,
    Literal,
    File,
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StyleElement {
    class: StyleElementClass,
    data: Cow<'static, str>,
}

impl StyleElement {
    pub fn from_link<Dt: Into<Cow<'static, str>>>(link: Dt) -> StyleElement {
        StyleElement::from_link_impl(link.into())
    }

    fn from_link_impl(link: Cow<'static, str>) -> StyleElement {
        StyleElement {
            class: StyleElementClass::Link,
            data: link,
        }
    }

    pub fn from_literal<Dt: Into<Cow<'static, str>>>(literal: Dt) -> StyleElement {
        StyleElement::from_literal_impl(literal.into())
    }

    fn from_literal_impl(literal: Cow<'static, str>) -> StyleElement {
        StyleElement {
            class: StyleElementClass::Literal,
            data: literal,
        }
    }

    pub fn from_path<Dt: Into<Cow<'static, str>>>(path: Dt) -> StyleElement {
        StyleElement::from_path_impl(path.into())
    }

    fn from_path_impl(path: Cow<'static, str>) -> StyleElement {
        StyleElement {
            class: StyleElementClass::File,
            data: path.into(),
        }
    }

    pub fn from_file(path: &(String, PathBuf)) -> Result<StyleElement, Error> {
        Ok(StyleElement {
            class: StyleElementClass::Literal,
            data: read_file(path, "literal style element from path")?.into(),
        })
    }

    pub fn load(&mut self, base: &(String, PathBuf)) -> Result<(), Error> {
        if self.class == StyleElementClass::File {
            self.data = read_file(&(format!("{}{}{}",
                                            base.0,
                                            if !['/', '\\'].contains(&(base.0.as_bytes()[base.0.as_bytes().len() - 1] as char)) &&
                                               !['/', '\\'].contains(&(self.data.as_bytes()[0] as char)) {
                                                "/"
                                            } else {
                                                ""
                                            },
                                            self.data),
                                    base.1.join(self.data.as_ref())),
                                  "file style element")
                ?
                .into();
            self.class = StyleElementClass::Literal;
        }

        Ok(())
    }
}

impl WrappedElement for StyleElement {
    fn head(&self) -> &str {
        match self.class {
            StyleElementClass::Link => &STYLE_LINK_HEAD,
            StyleElementClass::Literal |
            StyleElementClass::File => &STYLE_LITERAL_HEAD,
        }
    }

    fn content(&self) -> &str {
        &self.data
    }

    fn foot(&self) -> &str {
        match self.class {
            StyleElementClass::Link => &STYLE_LINK_FOOT,
            StyleElementClass::Literal |
            StyleElementClass::File => &STYLE_LITERAL_FOOT,
        }
    }
}


const STYLE_FIELDS: &[&str] = &["class", "data"];

struct StyleElementVisitor;

impl<'de> de::Visitor<'de> for StyleElementVisitor {
    type Value = StyleElement;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct StyleElement")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<StyleElement, E> {
        let mut itr = v.splitn(2, ":");
        Ok(match (itr.next(), itr.next()) {
            (Some(val), None) |
            (Some("literal"), Some(val)) => {
                StyleElement {
                    class: StyleElementClass::Literal,
                    data: val.to_string().into(),
                }
            }
            (Some("link"), Some(val)) => {
                StyleElement {
                    class: StyleElementClass::Link,
                    data: val.to_string().into(),
                }
            }
            (Some("file"), Some(val)) => {
                StyleElement {
                    class: StyleElementClass::File,
                    data: val.to_string().into(),
                }
            }

            (Some(tp), Some(_)) => return Err(de::Error::invalid_value(de::Unexpected::Str(tp), &r#""literal", "link", or "file""#)),
            (None, ..) => unreachable!(),
        })
    }

    fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<StyleElement, V::Error> {
        let mut class = None;
        let mut data = None;
        while let Some(key) = map.next_key()? {
            match key {
                "class" => {
                    if class.is_some() {
                        return Err(de::Error::duplicate_field("class"));
                    }
                    class = Some(match map.next_value()? {
                        "literal" => StyleElementClass::Literal,
                        "link" => StyleElementClass::Link,
                        "file" => StyleElementClass::File,
                        val => return Err(de::Error::invalid_value(de::Unexpected::Str(val), &r#""literal", "link", or "file""#)),
                    });
                }
                "data" => {
                    if data.is_some() {
                        return Err(de::Error::duplicate_field("data"));
                    }
                    data = Some(map.next_value()?);
                }
                _ => return Err(de::Error::unknown_field(key, STYLE_FIELDS)),
            }
        }

        Ok(StyleElement {
            class: class.ok_or_else(|| de::Error::missing_field("class"))?,
            data: data.ok_or_else(|| de::Error::missing_field("data"))?,
        })
    }
}

impl<'de> de::Deserialize<'de> for StyleElement {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_struct("StyleElement", STYLE_FIELDS, StyleElementVisitor)
    }
}
