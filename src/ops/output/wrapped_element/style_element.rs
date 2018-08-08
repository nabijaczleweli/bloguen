use self::super::super::super::super::util::{concat_path, read_file};
use self::super::super::super::super::Error;
use self::super::WrappedElement;
use std::path::PathBuf;
use std::borrow::Cow;
use serde::de;
use std::fmt;


lazy_static! {
    static ref STYLE_LINK_HEAD: &'static str = include_str!("../../../../assets/element_wrappers/style/link.head").trim();
    static ref STYLE_LINK_FOOT: &'static str = include_str!("../../../../assets/element_wrappers/style/link.foot").trim_left();

    static ref STYLE_LITERAL_HEAD: &'static str = include_str!("../../../../assets/element_wrappers/style/literal.head").trim_left();
    static ref STYLE_LITERAL_FOOT: &'static str = include_str!("../../../../assets/element_wrappers/style/literal.foot");
}



#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum StyleElementClass {
    Link,
    Literal,
    File,
}


/// A style specifier.
///
/// Can be a link or a literal, and a literal can be indirectly loaded from a file.
///
/// Consult the documentation for [`load()`](#fn.load) on handling filesystem interaxion.
///
/// # Deserialisation
///
/// There are two serialised forms, a verbose one:
///
/// ```
/// # extern crate toml;
/// # extern crate bloguen;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # use bloguen::ops::StyleElement;
/// #[derive(Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// struct StyleContainer {
///     pub styles: Vec<StyleElement>,
/// }
///
/// # fn main() {
/// let styles_toml = "[[styles]]
///                    class = 'link'
///                    data = '//nabijaczleweli.xyz/kaschism/assets/column.css'
///
///                    [[styles]]
///                    class = 'literal'
///                    data = '.indented { text-indent: 1em; }'
///
///                    [[styles]]
///                    class = 'file'
///                    data = 'common.css'";
///
/// let StyleContainer { styles } = toml::from_str(styles_toml).unwrap();
/// assert_eq!(&styles,
///            &[StyleElement::from_link("//nabijaczleweli.xyz/kaschism/assets/column.css"),
///              StyleElement::from_literal(".indented { text-indent: 1em; }"),
///              StyleElement::from_path("common.css")]);
/// # }
/// ```
///
/// And a compact one (the "literal" tag may be omitted if the content doesn't contain any colons):
///
/// ```
/// # extern crate toml;
/// # extern crate bloguen;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # use bloguen::ops::StyleElement;
/// #[derive(Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// struct StyleContainer {
///     pub styles: Vec<StyleElement>,
/// }
///
/// # fn main() {
/// let styles_toml = "styles = [
///                        'link://nabijaczleweli.xyz/kaschism/assets/column.css',
///                        'literal:.indented { text-indent: 1em; }',
///                        'file:common.css',
///                    ]";
///
/// let StyleContainer { styles } = toml::from_str(styles_toml).unwrap();
/// assert_eq!(&styles,
///            &[StyleElement::from_link("//nabijaczleweli.xyz/kaschism/assets/column.css"),
///              StyleElement::from_literal(".indented { text-indent: 1em; }"),
///              StyleElement::from_path("common.css")]);
/// # }
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StyleElement {
    class: StyleElementClass,
    data: Cow<'static, str>,
}

impl StyleElement {
    /// Create a style element linking to an external resource.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::{WrappedElement, StyleElement};
    /// let lonk = StyleElement::from_link("//nabijaczleweli.xyz/kaschism/assets/column.css");
    /// assert_eq!(
    ///     format!("{}{}{}", lonk.head(), lonk.content(), lonk.foot()),
    ///     "<link href=\"//nabijaczleweli.xyz/kaschism/assets/column.css\" rel=\"stylesheet\" />\n")
    /// ```
    pub fn from_link<Dt: Into<Cow<'static, str>>>(link: Dt) -> StyleElement {
        StyleElement::from_link_impl(link.into())
    }

    fn from_link_impl(link: Cow<'static, str>) -> StyleElement {
        StyleElement {
            class: StyleElementClass::Link,
            data: link,
        }
    }

    /// Create a style element including the specified literal literally.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::{WrappedElement, StyleElement};
    /// let lit = StyleElement::from_literal(".indented { text-indent: 1em; }");
    /// assert_eq!(format!("{}{}{}", lit.head(), lit.content(), lit.foot()),
    ///            "<style type=\"text/css\">\n\n.indented { text-indent: 1em; }\n\n</style>\n")
    /// ```
    pub fn from_literal<Dt: Into<Cow<'static, str>>>(literal: Dt) -> StyleElement {
        StyleElement::from_literal_impl(literal.into())
    }

    fn from_literal_impl(literal: Cow<'static, str>) -> StyleElement {
        StyleElement {
            class: StyleElementClass::Literal,
            data: literal,
        }
    }

    /// Create a style element pointing to the specified relative path.
    ///
    /// Consult [`load()`](#fn.load) documentation for more data.
    ///
    /// # Examples
    ///
    /// Given `$ROOT/common.css` containing:
    ///
    /// ```css
    /// ul, ol {
    ///     margin-top: 0;
    ///     margin-bottom: 0;
    /// }
    ///
    /// a > i.fa {
    ///     color: black;
    /// }
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::{WrappedElement, StyleElement};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # use bloguen::Error;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-output-wrapped_element-style_element-from_path");
    /// # fs::create_dir_all(&root).unwrap();
    /// # File::create(root.join("common.css")).unwrap().write_all("\
    /// #     ul, ol {\n\
    /// #         margin-top: 0;\n\
    /// #         margin-bottom: 0;\n\
    /// #     }\n\
    /// #     \n\
    /// #     a > i.fa {\n\
    /// #         color: black;\n\
    /// #     }\n
    /// # ".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    ///
    /// let mut lit_p = StyleElement::from_path("common.css");
    /// assert_eq!(lit_p.load(&("$ROOT".to_string(), root.clone())), Ok(()));
    /// assert_eq!(format!("{}{}{}", lit_p.head(), lit_p.content(), lit_p.foot()),
    /// "<style type=\"text/css\">\n\n\
    ///      ul, ol {\n\
    ///          margin-top: 0;\n\
    ///          margin-bottom: 0;\n\
    ///      }\n\
    ///      \n\
    ///      a > i.fa {\n\
    ///          color: black;\n\
    ///      }\n\n\
    /// \n\n</style>\n");
    /// ```
    pub fn from_path<Dt: Into<Cow<'static, str>>>(path: Dt) -> StyleElement {
        StyleElement::from_path_impl(path.into())
    }

    fn from_path_impl(path: Cow<'static, str>) -> StyleElement {
        StyleElement {
            class: StyleElementClass::File,
            data: path.into(),
        }
    }


    /// Create a literal style element from the contents of the specified file.
    ///
    /// # Examples
    ///
    /// Given `$ROOT/common.css` containing:
    ///
    /// ```css
    /// ul, ol {
    ///     margin-top: 0;
    ///     margin-bottom: 0;
    /// }
    ///
    /// a > i.fa {
    ///     color: black;
    /// }
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::{WrappedElement, StyleElement};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # use bloguen::Error;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-output-wrapped_element-style_element-from_file");
    /// # fs::create_dir_all(&root).unwrap();
    /// # File::create(root.join("common.css")).unwrap().write_all("\
    /// #     ul, ol {\n\
    /// #         margin-top: 0;\n\
    /// #         margin-bottom: 0;\n\
    /// #     }\n\
    /// #     \n\
    /// #     a > i.fa {\n\
    /// #         color: black;\n\
    /// #     }\n
    /// # ".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    ///
    /// let lit_p = StyleElement::from_file(&("$ROOT/common.cs".to_string(), root.join("common.css"))).unwrap();
    /// assert_eq!(format!("{}{}{}", lit_p.head(), lit_p.content(), lit_p.foot()), "\
    /// <style type=\"text/css\">\n\n\
    ///      ul, ol {\n\
    ///          margin-top: 0;\n\
    ///          margin-bottom: 0;\n\
    ///      }\n\
    ///      \n\
    ///      a > i.fa {\n\
    ///          color: black;\n\
    ///      }\n\n\
    /// \n\n</style>\n");
    /// ```
    pub fn from_file(path: &(String, PathBuf)) -> Result<StyleElement, Error> {
        Ok(StyleElement {
            class: StyleElementClass::Literal,
            data: read_file(path, "literal style element from path")?.into(),
        })
    }

    /// Read data from the filesystem, if appropriate.
    ///
    /// Path elements are concatenated with the specified root, then [`read_file()`](../util/fn.read_file.html)d in, becoming
    /// literals.
    ///
    /// Non-path elements are unaffected.
    ///
    /// # Examples
    ///
    /// Given the following directory layout:
    ///
    /// ```plaintext
    /// $ROOT
    ///   common.css
    ///   assets
    ///     effects.css
    /// ```
    ///
    /// Given `$ROOT/common.css` containing:
    ///
    /// ```css
    /// ul, ol {
    ///     margin-top: 0;
    ///     margin-bottom: 0;
    /// }
    ///
    /// a > i.fa {
    ///     color: black;
    /// }
    /// ```
    ///
    /// Given `$ROOT/assets/effects.css` containing:
    ///
    /// ```css
    /// .ruby {
    ///     /* That's Ruby according to https://en.wikipedia.org/wiki/Ruby_(color). */
    ///     color: #E0115F;
    /// }
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::StyleElement;
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # use bloguen::Error;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-output-wrapped_element-style_element-load");
    /// # fs::create_dir_all(root.join("assets")).unwrap();
    /// # File::create(root.join("common.css")).unwrap().write_all("\
    /// #     ul, ol {\n\
    /// #         margin-top: 0;\n\
    /// #         margin-bottom: 0;\n\
    /// #     }\n\
    /// #     \n\
    /// #     a > i.fa {\n\
    /// #         color: black;\n\
    /// #     }\n
    /// # ".as_bytes()).unwrap();
    /// # File::create(root.join("assets").join("effects.css")).unwrap().write_all("\
    /// #     .ruby {\n\
    /// #         /* That's Ruby according to https://en.wikipedia.org/wiki/Ruby_(color). */\n\
    /// #         color: #E0115F;\n\
    /// #     }\n
    /// # ".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    ///
    /// let mut elem = StyleElement::from_path("common.css");
    /// assert_eq!(elem.load(&("$ROOT".to_string(), root.clone())), Ok(()));
    /// assert_eq!(elem, StyleElement::from_literal("\
    ///     ul, ol {\n\
    ///         margin-top: 0;\n\
    ///         margin-bottom: 0;\n\
    ///     }\n\
    ///     \n\
    ///     a > i.fa {\n\
    ///         color: black;\n\
    ///     }\n
    /// "));
    ///
    /// let mut elem = StyleElement::from_path("assets/.././assets/effects.css");
    /// assert_eq!(elem.load(&("$ROOT".to_string(), root.clone())), Ok(()));
    /// assert_eq!(elem, StyleElement::from_literal("\
    ///    .ruby {\n\
    ///         /* That's Ruby according to https://en.wikipedia.org/wiki/Ruby_(color). */\n\
    ///         color: #E0115F;\n\
    ///     }\n
    /// "));
    ///
    /// let mut elem = StyleElement::from_path("assets/nonexistant.css");
    /// assert_eq!(elem.load(&("$ROOT".to_string(), root.clone())), Err(Error::FileNotFound {
    ///     who: "file style element",
    ///     path: "$ROOT/assets/nonexistant.css".into(),
    /// }));
    /// assert_eq!(elem, StyleElement::from_path("assets/nonexistant.css"));
    /// ```
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
                                    concat_path(base.1.clone(), &self.data)),
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
            StyleElementClass::Literal => &STYLE_LITERAL_HEAD,
            StyleElementClass::File => "&lt;",
        }
    }

    fn content(&self) -> &str {
        &self.data
    }

    fn foot(&self) -> &str {
        match self.class {
            StyleElementClass::Link => &STYLE_LINK_FOOT,
            StyleElementClass::Literal => &STYLE_LITERAL_FOOT,
            StyleElementClass::File => "&gt;\n",
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
