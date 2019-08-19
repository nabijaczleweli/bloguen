use self::super::super::super::super::util::{concat_path, read_file};
use self::super::super::super::super::Error;
use self::super::WrappedElement;
use std::path::PathBuf;
use std::borrow::Cow;
use serde::de;
use std::fmt;


lazy_static! {
    static ref SCRIPT_LINK_HEAD: &'static str = include_str!("../../../../assets/element_wrappers/script/link.head").trim();
    static ref SCRIPT_LINK_FOOT: &'static str = include_str!("../../../../assets/element_wrappers/script/link.foot").trim_start();

    static ref SCRIPT_LITERAL_HEAD: &'static str = include_str!("../../../../assets/element_wrappers/script/literal.head").trim_start();
    static ref SCRIPT_LITERAL_FOOT: &'static str = include_str!("../../../../assets/element_wrappers/script/literal.foot");
}



#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum ScriptElementClass {
    Link,
    Literal,
    File,
}


/// A script specifier.
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
/// # use bloguen::ops::ScriptElement;
/// #[derive(Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// struct ScriptContainer {
///     pub styles: Vec<ScriptElement>,
/// }
///
/// # fn main() {
/// let styles_toml = "[[styles]]
///                    class = 'link'
///                    data = '/content/assets/syllable.js'
///
///                    [[styles]]
///                    class = 'literal'
///                    data = 'document.getElementById(\"title\").innerText = \"Наган\";'
///
///                    [[styles]]
///                    class = 'file'
///                    data = 'MathJax-config.js'";
///
/// let ScriptContainer { styles } = toml::from_str(styles_toml).unwrap();
/// assert_eq!(&styles,
///            &[ScriptElement::from_link("/content/assets/syllable.js"),
///              ScriptElement::from_literal("document.getElementById(\"title\").innerText = \"Наган\";"),
///              ScriptElement::from_path("MathJax-config.js")]);
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
/// # use bloguen::ops::ScriptElement;
/// #[derive(Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// struct ScriptContainer {
///     pub styles: Vec<ScriptElement>,
/// }
///
/// # fn main() {
/// let styles_toml = "styles = [
///                        'link:/content/assets/syllable.js',
///                        'literal:document.getElementById(\"title\").innerText = \"Наган\";',
///                        'file:MathJax-config.js',
///                    ]";
///
/// let ScriptContainer { styles } = toml::from_str(styles_toml).unwrap();
/// assert_eq!(&styles,
///            &[ScriptElement::from_link("/content/assets/syllable.js"),
///              ScriptElement::from_literal("document.getElementById(\"title\").innerText = \"Наган\";"),
///              ScriptElement::from_path("MathJax-config.js")]);
/// # }
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScriptElement {
    class: ScriptElementClass,
    data: Cow<'static, str>,
}

impl ScriptElement {
    /// Create a script element linking to an external resource.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::{WrappedElement, ScriptElement};
    /// let lonk = ScriptElement::from_link("/content/assets/syllable.js");
    /// assert_eq!(
    ///     format!("{}{}{}", lonk.head(), lonk.content(), lonk.foot()),
    ///     "<script type=\"text/javascript\" src=\"/content/assets/syllable.js\"></script>\n")
    /// ```
    pub fn from_link<Dt: Into<Cow<'static, str>>>(link: Dt) -> ScriptElement {
        ScriptElement::from_link_impl(link.into())
    }

    fn from_link_impl(link: Cow<'static, str>) -> ScriptElement {
        ScriptElement {
            class: ScriptElementClass::Link,
            data: link,
        }
    }

    /// Create a script element including the specified literal literally.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::{WrappedElement, ScriptElement};
    /// let lit = ScriptElement::from_literal("document.getElementById(\"title\").innerText = \"Наган\";");
    /// assert_eq!(
    ///     format!("{}{}{}", lit.head(), lit.content(), lit.foot()),
    ///     "<script type=\"text/javascript\">\n\ndocument.getElementById(\"title\").innerText = \"Наган\";\n\n</script>\n")
    /// ```
    pub fn from_literal<Dt: Into<Cow<'static, str>>>(literal: Dt) -> ScriptElement {
        ScriptElement::from_literal_impl(literal.into())
    }

    fn from_literal_impl(literal: Cow<'static, str>) -> ScriptElement {
        ScriptElement {
            class: ScriptElementClass::Literal,
            data: literal,
        }
    }

    /// Create a script element pointing to the specified relative path.
    ///
    /// Consult [`load()`](#fn.load) documentation for more data.
    ///
    /// # Examples
    ///
    /// Given `$ROOT/MathJax-config.js` containing:
    ///
    /// ```js
    /// MathJax.Hub.Config({
    ///   jax: ["input/AsciiMath", "output/HTML-CSS"],
    ///   extensions: ["asciimath2jax.js"],
    ///   asciimath2jax: {
    ///     delimiters: [['[​[​', '​]​]']],
    ///     preview: "[[maths]]"
    ///   },
    ///   AsciiMath: {
    ///     decimal: "."
    ///   },
    ///   "HTML-CSS": {
    ///     undefinedFamily: "STIXGeneral,'DejaVu Sans Mono','Arial Unicode MS',serif"
    ///   }
    /// });
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::{WrappedElement, ScriptElement};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # use bloguen::Error;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-output-wrapped_element-script_element-from_path");
    /// # fs::create_dir_all(&root).unwrap();
    /// # File::create(root.join("MathJax-config.js")).unwrap().write_all("\
    /// #     MathJax.Hub.Config({\n\
    /// #       jax: [\"input/AsciiMath\", \"output/HTML-CSS\"],\n\
    /// #       extensions: [\"asciimath2jax.js\"],\n\
    /// #       asciimath2jax: {\n\
    /// #         delimiters: [['[​[​', '​]​]']],\n\
    /// #         preview: \"[[maths]]\"\n\
    /// #       },\n\
    /// #       AsciiMath: {\n\
    /// #         decimal: \".\"\n\
    /// #       },\n\
    /// #       \"HTML-CSS\": {\n\
    /// #         undefinedFamily: \"STIXGeneral,'DejaVu Sans Mono','Arial Unicode MS',serif\"\n\
    /// #       }\n\
    /// #     });\n\
    /// # ".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    ///
    /// let mut lit_p = ScriptElement::from_path("MathJax-config.js");
    /// assert_eq!(lit_p.load(&("$ROOT".to_string(), root.clone())), Ok(()));
    /// assert_eq!(format!("{}{}{}", lit_p.head(), lit_p.content(), lit_p.foot()),
    /// "<script type=\"text/javascript\">\n\n\
    ///     MathJax.Hub.Config({\n\
    ///       jax: [\"input/AsciiMath\", \"output/HTML-CSS\"],\n\
    ///       extensions: [\"asciimath2jax.js\"],\n\
    ///       asciimath2jax: {\n\
    ///         delimiters: [['[​[​', '​]​]']],\n\
    ///         preview: \"[[maths]]\"\n\
    ///       },\n\
    ///       AsciiMath: {\n\
    ///         decimal: \".\"\n\
    ///       },\n\
    ///       \"HTML-CSS\": {\n\
    ///         undefinedFamily: \"STIXGeneral,'DejaVu Sans Mono','Arial Unicode MS',serif\"\n\
    ///       }\n\
    ///     });\n\
    /// \n\n</script>\n");
    /// ```
    pub fn from_path<Dt: Into<Cow<'static, str>>>(path: Dt) -> ScriptElement {
        ScriptElement::from_path_impl(path.into())
    }

    fn from_path_impl(path: Cow<'static, str>) -> ScriptElement {
        ScriptElement {
            class: ScriptElementClass::File,
            data: path.into(),
        }
    }


    /// Create a literal script element from the contents of the specified file.
    ///
    /// # Examples
    ///
    /// Given `$ROOT/MathJax-config.js` containing:
    ///
    /// ```js
    /// MathJax.Hub.Config({
    ///   jax: ["input/AsciiMath", "output/HTML-CSS"],
    ///   extensions: ["asciimath2jax.js"],
    ///   asciimath2jax: {
    ///     delimiters: [['[​[​', '​]​]']],
    ///     preview: "[[maths]]"
    ///   },
    ///   AsciiMath: {
    ///     decimal: "."
    ///   },
    ///   "HTML-CSS": {
    ///     undefinedFamily: "STIXGeneral,'DejaVu Sans Mono','Arial Unicode MS',serif"
    ///   }
    /// });
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::{WrappedElement, ScriptElement};
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # use bloguen::Error;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-output-wrapped_element-script_element-from_file");
    /// # fs::create_dir_all(&root).unwrap();
    /// # File::create(root.join("MathJax-config.js")).unwrap().write_all("\
    /// #     MathJax.Hub.Config({\n\
    /// #       jax: [\"input/AsciiMath\", \"output/HTML-CSS\"],\n\
    /// #       extensions: [\"asciimath2jax.js\"],\n\
    /// #       asciimath2jax: {\n\
    /// #         delimiters: [['[​[​', '​]​]']],\n\
    /// #         preview: \"[[maths]]\"\n\
    /// #       },\n\
    /// #       AsciiMath: {\n\
    /// #         decimal: \".\"\n\
    /// #       },\n\
    /// #       \"HTML-CSS\": {\n\
    /// #         undefinedFamily: \"STIXGeneral,'DejaVu Sans Mono','Arial Unicode MS',serif\"\n\
    /// #       }\n\
    /// #     });\n\
    /// # ".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    ///
    /// let lit_p = ScriptElement::from_file(&("$ROOT/common.cs".to_string(), root.join("MathJax-config.js"))).unwrap();
    /// assert_eq!(format!("{}{}{}", lit_p.head(), lit_p.content(), lit_p.foot()), "\
    /// <script type=\"text/javascript\">\n\n\
    ///     MathJax.Hub.Config({\n\
    ///       jax: [\"input/AsciiMath\", \"output/HTML-CSS\"],\n\
    ///       extensions: [\"asciimath2jax.js\"],\n\
    ///       asciimath2jax: {\n\
    ///         delimiters: [['[​[​', '​]​]']],\n\
    ///         preview: \"[[maths]]\"\n\
    ///       },\n\
    ///       AsciiMath: {\n\
    ///         decimal: \".\"\n\
    ///       },\n\
    ///       \"HTML-CSS\": {\n\
    ///         undefinedFamily: \"STIXGeneral,'DejaVu Sans Mono','Arial Unicode MS',serif\"\n\
    ///       }\n\
    ///     });\n\
    /// \n\n</script>\n");
    /// ```
    pub fn from_file(path: &(String, PathBuf)) -> Result<ScriptElement, Error> {
        Ok(ScriptElement {
            class: ScriptElementClass::Literal,
            data: read_file(path, "literal script element from path")?.into(),
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
    ///   MathJax-config.js
    ///   assets
    ///     octicons.js
    /// ```
    ///
    /// Given `$ROOT/MathJax-config.js` containing:
    ///
    /// ```js
    /// MathJax.Hub.Config({
    ///   jax: ["input/AsciiMath", "output/HTML-CSS"],
    ///   extensions: ["asciimath2jax.js"],
    ///   asciimath2jax: {
    ///     delimiters: [['[​[​', '​]​]']],
    ///     preview: "[[maths]]"
    ///   },
    ///   AsciiMath: {
    ///     decimal: "."
    ///   },
    ///   "HTML-CSS": {
    ///     undefinedFamily: "STIXGeneral,'DejaVu Sans Mono','Arial Unicode MS',serif"
    ///   }
    /// });
    /// ```
    ///
    /// Given `$ROOT/assets/octicons.js` containing:
    ///
    /// ```js
    /// .ruby {
    ///     /* That's Ruby according to https://en.wikipedia.org/wiki/Ruby_(color). */
    ///     color: #E0115F;
    /// }
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::ScriptElement;
    /// # use std::fs::{self, File};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # use bloguen::Error;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-output-wrapped_element-script_element-load");
    /// # fs::create_dir_all(root.join("assets")).unwrap();
    /// # File::create(root.join("MathJax-config.js")).unwrap().write_all("\
    /// #     MathJax.Hub.Config({\n\
    /// #       jax: [\"input/AsciiMath\", \"output/HTML-CSS\"],\n\
    /// #       extensions: [\"asciimath2jax.js\"],\n\
    /// #       asciimath2jax: {\n\
    /// #         delimiters: [['[​[​', '​]​]']],\n\
    /// #         preview: \"[[maths]]\"\n\
    /// #       },\n\
    /// #       AsciiMath: {\n\
    /// #         decimal: \".\"\n\
    /// #       },\n\
    /// #       \"HTML-CSS\": {\n\
    /// #         undefinedFamily: \"STIXGeneral,'DejaVu Sans Mono','Arial Unicode MS',serif\"\n\
    /// #       }\n\
    /// #     });\n\
    /// # ".as_bytes()).unwrap();
    /// # File::create(root.join("assets").join("octicons.js")).unwrap().write_all("\
    /// #     window.addEventListener(\"load\", function() {\n\
    /// #         const PLACEHOLDER = document.getElementById(\"octicons-placeholder\");\n\
    /// #         \n\
    /// #         const request = new XMLHttpRequest();\n\
    /// #         request.open(\"GET\", \"/content/assets/octicons/sprite.octicons.svg\");\n\
    /// #         request.onload = function(load) {\n\
    /// #             PLACEHOLDER.outerHTML = load.target.responseText.replace(\"<svg\", \"<svg class=\\\"hidden\\\"\");\n\
    /// #         };\n\
    /// #         request.send();\n\
    /// #     });\n\
    /// # ".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    ///
    /// let mut elem = ScriptElement::from_path("MathJax-config.js");
    /// assert_eq!(elem.load(&("$ROOT".to_string(), root.clone())), Ok(()));
    /// assert_eq!(elem, ScriptElement::from_literal("\
    ///     MathJax.Hub.Config({\n\
    ///       jax: [\"input/AsciiMath\", \"output/HTML-CSS\"],\n\
    ///       extensions: [\"asciimath2jax.js\"],\n\
    ///       asciimath2jax: {\n\
    ///         delimiters: [['[​[​', '​]​]']],\n\
    ///         preview: \"[[maths]]\"\n\
    ///       },\n\
    ///       AsciiMath: {\n\
    ///         decimal: \".\"\n\
    ///       },\n\
    ///       \"HTML-CSS\": {\n\
    ///         undefinedFamily: \"STIXGeneral,'DejaVu Sans Mono','Arial Unicode MS',serif\"\n\
    ///       }\n\
    ///     });\n\
    /// "));
    ///
    /// let mut elem = ScriptElement::from_path("assets/.././assets/octicons.js");
    /// assert_eq!(elem.load(&("$ROOT".to_string(), root.clone())), Ok(()));
    /// assert_eq!(elem, ScriptElement::from_literal("\
    ///     window.addEventListener(\"load\", function() {\n\
    ///         const PLACEHOLDER = document.getElementById(\"octicons-placeholder\");\n\
    ///         \n\
    ///         const request = new XMLHttpRequest();\n\
    ///         request.open(\"GET\", \"/content/assets/octicons/sprite.octicons.svg\");\n\
    ///         request.onload = function(load) {\n\
    ///             PLACEHOLDER.outerHTML = load.target.responseText.replace(\"<svg\", \"<svg class=\\\"hidden\\\"\");\n\
    ///         };\n\
    ///         request.send();\n\
    ///     });\n\
    /// "));
    ///
    /// let mut elem = ScriptElement::from_path("assets/nonexistant.css");
    /// assert_eq!(elem.load(&("$ROOT".to_string(), root.clone())), Err(Error::FileNotFound {
    ///     who: "file script element",
    ///     path: "$ROOT/assets/nonexistant.css".into(),
    /// }));
    /// assert_eq!(elem, ScriptElement::from_path("assets/nonexistant.css"));
    /// ```
    pub fn load(&mut self, base: &(String, PathBuf)) -> Result<(), Error> {
        if self.class == ScriptElementClass::File {
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
                                  "file script element")
                ?
                .into();
            self.class = ScriptElementClass::Literal;
        }

        Ok(())
    }
}

impl WrappedElement for ScriptElement {
    fn head(&self) -> &str {
        match self.class {
            ScriptElementClass::Link => &SCRIPT_LINK_HEAD,
            ScriptElementClass::Literal => &SCRIPT_LITERAL_HEAD,
            ScriptElementClass::File => "&lt;",
        }
    }

    fn content(&self) -> &str {
        &self.data
    }

    fn foot(&self) -> &str {
        match self.class {
            ScriptElementClass::Link => &SCRIPT_LINK_FOOT,
            ScriptElementClass::Literal => &SCRIPT_LITERAL_FOOT,
            ScriptElementClass::File => "&gt;\n",
        }
    }
}


const SCRIPT_FIELDS: &[&str] = &["class", "data"];

struct ScriptElementVisitor;

impl<'de> de::Visitor<'de> for ScriptElementVisitor {
    type Value = ScriptElement;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct ScriptElement")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<ScriptElement, E> {
        let mut itr = v.splitn(2, ":");
        Ok(match (itr.next(), itr.next()) {
            (Some(val), None) |
            (Some("literal"), Some(val)) => {
                ScriptElement {
                    class: ScriptElementClass::Literal,
                    data: val.to_string().into(),
                }
            }
            (Some("link"), Some(val)) => {
                ScriptElement {
                    class: ScriptElementClass::Link,
                    data: val.to_string().into(),
                }
            }
            (Some("file"), Some(val)) => {
                ScriptElement {
                    class: ScriptElementClass::File,
                    data: val.to_string().into(),
                }
            }

            (Some(tp), Some(_)) => return Err(de::Error::invalid_value(de::Unexpected::Str(tp), &r#""literal", "link", or "file""#)),
            (None, ..) => unreachable!(),
        })
    }

    fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<ScriptElement, V::Error> {
        let mut class = None;
        let mut data = None;
        while let Some(key) = map.next_key()? {
            match key {
                "class" => {
                    if class.is_some() {
                        return Err(de::Error::duplicate_field("class"));
                    }
                    class = Some(match map.next_value()? {
                        "literal" => ScriptElementClass::Literal,
                        "link" => ScriptElementClass::Link,
                        "file" => ScriptElementClass::File,
                        val => return Err(de::Error::invalid_value(de::Unexpected::Str(val), &r#""literal", "link", or "file""#)),
                    });
                }
                "data" => {
                    if data.is_some() {
                        return Err(de::Error::duplicate_field("data"));
                    }
                    data = Some(map.next_value()?);
                }
                _ => return Err(de::Error::unknown_field(key, SCRIPT_FIELDS)),
            }
        }

        Ok(ScriptElement {
            class: class.ok_or_else(|| de::Error::missing_field("class"))?,
            data: data.ok_or_else(|| de::Error::missing_field("data"))?,
        })
    }
}

impl<'de> de::Deserialize<'de> for ScriptElement {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_struct("ScriptElement", SCRIPT_FIELDS, ScriptElementVisitor)
    }
}
