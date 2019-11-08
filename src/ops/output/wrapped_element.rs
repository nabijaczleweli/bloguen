use self::super::super::super::util::{concat_path, read_file};
use std::path::{PathBuf, is_separator as is_path_separator};
use self::super::super::super::Error;
use std::marker::PhantomData;
use std::borrow::Cow;
use serde::de;
use std::fmt;


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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum ElementClass {
    Link,
    Literal,
    File,
}


/// A script specifier.
///
/// Can be a link or a literal, and a literal can be indirectly loaded from a file.
///
/// Consult the documentation for [`load()`](struct.WrappedElementImpl.html#fn.load) on handling filesystem interaxion.
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
///     pub script: Vec<ScriptElement>,
/// }
///
/// # fn main() {
/// let script_toml =
///     "[[script]]
///      class = 'link'
///      data = '/content/assets/syllable.js'
///
///      [[script]]
///      class = 'literal'
///      data = 'document.getElementById(\"title\").innerText = \"Наган\";'
///
///      [[script]]
///      class = 'file'
///      data = 'MathJax-config.js'";
///
/// let ScriptContainer { script } = toml::from_str(script_toml).unwrap();
/// assert_eq!(&script,
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
///     pub scripts: Vec<ScriptElement>,
/// }
///
/// # fn main() {
/// let scripts_toml =
///     "scripts = [
///          'link:/content/assets/syllable.js',
///          'literal:document.getElementById(\"title\").innerText = \"Наган\";',
///          'file:MathJax-config.js',
///      ]";
///
/// let ScriptContainer { scripts } = toml::from_str(scripts_toml).unwrap();
/// assert_eq!(&scripts,
///            &[ScriptElement::from_link("/content/assets/syllable.js"),
///              ScriptElement::from_literal("document.getElementById(\"title\").innerText = \"Наган\";"),
///              ScriptElement::from_path("MathJax-config.js")]);
/// # }
/// ```
pub type ScriptElement = WrappedElementImpl<WrappedElementImplDataScript>;

/// A style specifier.
///
/// Can be a link or a literal, and a literal can be indirectly loaded from a file.
///
/// Consult the documentation for [`load()`](struct.WrappedElementImpl.html#fn.load) on handling filesystem interaxion.
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
///     pub style: Vec<StyleElement>,
/// }
///
/// # fn main() {
/// let style_toml =
///     "[[style]]
///      class = 'link'
///      data = '//nabijaczleweli.xyz/kaschism/assets/column.css'
///
///      [[style]]
///      class = 'literal'
///      data = '.indented { text-indent: 1em; }'
///
///      [[style]]
///      class = 'file'
///      data = 'common.css'";
///
/// let StyleContainer { style } = toml::from_str(style_toml).unwrap();
/// assert_eq!(&style,
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
/// let styles_toml =
///     "styles = [
///          'link://nabijaczleweli.xyz/kaschism/assets/column.css',
///          'literal:.indented { text-indent: 1em; }',
///          'file:common.css',
///      ]";
///
/// let StyleContainer { styles } = toml::from_str(styles_toml).unwrap();
/// assert_eq!(&styles,
///            &[StyleElement::from_link("//nabijaczleweli.xyz/kaschism/assets/column.css"),
///              StyleElement::from_literal(".indented { text-indent: 1em; }"),
///              StyleElement::from_path("common.css")]);
/// # }
/// ```
pub type StyleElement = WrappedElementImpl<WrappedElementImplDataStyle>;


/// A semi-generic wrapped data specifier, backing [`ScriptElement`](type.ScriptElement.html) and
/// [`StyleElement`](type.StyleElement.html).
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrappedElementImpl<Dt: WrappedElementImplData>(WrappedElementImplImpl, PhantomData<Dt>);

impl<Dt: WrappedElementImplData> WrappedElementImpl<Dt> {
    /// Create an element linking to an external resource.
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
    ///
    /// ```
    /// # use bloguen::ops::{WrappedElement, StyleElement};
    /// let lonk = StyleElement::from_link("//nabijaczleweli.xyz/kaschism/assets/column.css");
    /// assert_eq!(
    ///     format!("{}{}{}", lonk.head(), lonk.content(), lonk.foot()),
    ///     "<link href=\"//nabijaczleweli.xyz/kaschism/assets/column.css\" rel=\"stylesheet\" />\n")
    /// ```
    pub fn from_link<DtF: Into<Cow<'static, str>>>(link: DtF) -> Self {
        Self(WrappedElementImplImpl::from_link_impl(link.into()), PhantomData)
    }


    /// Create an element including the specified literal literally.
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
    ///
    /// ```
    /// # use bloguen::ops::{WrappedElement, StyleElement};
    /// let lit = StyleElement::from_literal(".indented { text-indent: 1em; }");
    /// assert_eq!(
    ///     format!("{}{}{}", lit.head(), lit.content(), lit.foot()),
    ///     "<style type=\"text/css\">\n\n.indented { text-indent: 1em; }\n\n</style>\n")
    /// ```
    pub fn from_literal<DtF: Into<Cow<'static, str>>>(literal: DtF) -> Self {
        Self(WrappedElementImplImpl::from_literal_impl(literal.into()), PhantomData)
    }

    /// Create an element pointing to the specified relative path.
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
    pub fn from_path<DtF: Into<Cow<'static, str>>>(path: DtF) -> Self {
        Self(WrappedElementImplImpl::from_path_impl(path.into()), PhantomData)
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
    /// let lit_p = ScriptElement::from_file(&("$ROOT/MathJax-config.js".to_string(), root.join("MathJax-config.js"))).unwrap();
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
    /// let lit_p = StyleElement::from_file(&("$ROOT/common.css".to_string(), root.join("common.css"))).unwrap();
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
    pub fn from_file(path: &(String, PathBuf)) -> Result<Self, Error> {
        WrappedElementImplImpl::from_file(path.into(), Dt::from_file_what_for()).map(|dt| Self(dt, PhantomData))
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
    /// window.addEventListener("load", function() {
    ///     const PLACEHOLDER = document.getElementById("octicons-placeholder");
    ///
    ///     const request = new XMLHttpRequest();
    ///     request.open("GET", "/content/assets/octicons/sprite.octicons.svg");
    ///     request.onload = function(load) {
    ///         PLACEHOLDER.outerHTML = load.target.responseText.replace("<svg", "<svg class=\"hidden\"");
    ///     };
    ///     request.send();
    /// });
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
    /// # File::create(root.join("assets").join("octicons.js")).unwrap().write_all("window.addEventListener(\"load\",
    /// function() {\n\
    /// #     const PLACEHOLDER = document.getElementById(\"octicons-placeholder\");\n\
    /// #     \n\
    /// #     const request = new XMLHttpRequest();\n\
    /// #     request.open(\"GET\", \"/content/assets/octicons/sprite.octicons.svg\");\n\
    /// #     request.onload = function(load) {\n\
    /// #         PLACEHOLDER.outerHTML = load.target.responseText.replace(\"<svg\", \"<svg class=\\\"hidden\\\"\");\n\
    /// #     };\n\
    /// #     request.send();\n\
    /// # });\n\
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
    /// window.addEventListener(\"load\", function() {\n\
    ///     const PLACEHOLDER = document.getElementById(\"octicons-placeholder\");\n\
    ///     \n\
    ///     const request = new XMLHttpRequest();\n\
    ///     request.open(\"GET\", \"/content/assets/octicons/sprite.octicons.svg\");\n\
    ///     request.onload = function(load) {\n\
    ///         PLACEHOLDER.outerHTML = load.target.responseText.replace(\"<svg\", \"<svg class=\\\"hidden\\\"\");\n\
    ///     };\n\
    ///     request.send();\n\
    /// });\n\
    /// "));
    ///
    /// let mut elem = ScriptElement::from_path("assets/nonexistant.js");
    /// assert_eq!(elem.load(&("$ROOT".to_string(), root.clone())), Err(Error::FileNotFound {
    ///     who: "file script element",
    ///     path: "$ROOT/assets/nonexistant.js".into(),
    /// }));
    /// assert_eq!(elem, ScriptElement::from_path("assets/nonexistant.js"));
    /// ```
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
    /// # File::create(root.join("assets").join("effects.css")).unwrap().write_all(".ruby {\n\
    /// #     /* That's Ruby according to https://en.wikipedia.org/wiki/Ruby_(color). */\n\
    /// #     color: #E0115F;\n\
    /// # }\n
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
        self.0.load(base, Dt::file_load_what_for())
    }
}



lazy_static! {
    static ref SCRIPT_LINK_HEAD: &'static str = include_str!("../../../assets/element_wrappers/script/link.head").trim();
    static ref SCRIPT_LINK_FOOT: &'static str = include_str!("../../../assets/element_wrappers/script/link.foot").trim_start();

    static ref SCRIPT_LITERAL_HEAD: &'static str = include_str!("../../../assets/element_wrappers/script/literal.head").trim_start();
    static ref SCRIPT_LITERAL_FOOT: &'static str = include_str!("../../../assets/element_wrappers/script/literal.foot");


    static ref STYLE_LINK_HEAD: &'static str = include_str!("../../../assets/element_wrappers/style/link.head").trim();
    static ref STYLE_LINK_FOOT: &'static str = include_str!("../../../assets/element_wrappers/style/link.foot").trim_start();

    static ref STYLE_LITERAL_HEAD: &'static str = include_str!("../../../assets/element_wrappers/style/literal.head").trim_start();
    static ref STYLE_LITERAL_FOOT: &'static str = include_str!("../../../assets/element_wrappers/style/literal.foot");
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrappedElementImplDataScript;
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrappedElementImplDataStyle;

pub trait WrappedElementImplData {
    fn link_head() -> &'static str;
    fn literal_head() -> &'static str;

    fn link_foot() -> &'static str;
    fn literal_foot() -> &'static str;

    fn user_facing_name() -> &'static str;
    fn file_load_what_for() -> &'static str;
    fn from_file_what_for() -> &'static str;
}

impl WrappedElementImplData for WrappedElementImplDataScript {
    fn link_head() -> &'static str {
        &SCRIPT_LINK_HEAD
    }

    fn literal_head() -> &'static str {
        &SCRIPT_LITERAL_HEAD
    }

    fn link_foot() -> &'static str {
        &SCRIPT_LINK_FOOT
    }

    fn literal_foot() -> &'static str {
        &SCRIPT_LITERAL_FOOT
    }

    fn user_facing_name() -> &'static str {
        "ScriptElement"
    }

    fn file_load_what_for() -> &'static str {
        "file script element"
    }

    fn from_file_what_for() -> &'static str {
        "literal script element from path"
    }
}

impl WrappedElementImplData for WrappedElementImplDataStyle {
    fn link_head() -> &'static str {
        &STYLE_LINK_HEAD
    }

    fn literal_head() -> &'static str {
        &STYLE_LITERAL_HEAD
    }

    fn link_foot() -> &'static str {
        &STYLE_LINK_FOOT
    }

    fn literal_foot() -> &'static str {
        &STYLE_LITERAL_FOOT
    }

    fn user_facing_name() -> &'static str {
        "StyleElement"
    }

    fn file_load_what_for() -> &'static str {
        "file style element"
    }

    fn from_file_what_for() -> &'static str {
        "literal style element from path"
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrappedElementImplImpl {
    class: ElementClass,
    data: Cow<'static, str>,
}

impl WrappedElementImplImpl {
    fn from_link_impl(link: Cow<'static, str>) -> Self {
        Self {
            class: ElementClass::Link,
            data: link,
        }
    }

    fn from_literal_impl(literal: Cow<'static, str>) -> Self {
        Self {
            class: ElementClass::Literal,
            data: literal,
        }
    }

    fn from_path_impl(path: Cow<'static, str>) -> Self {
        Self {
            class: ElementClass::File,
            data: path.into(),
        }
    }

    fn from_file(path: &(String, PathBuf), what_for: &'static str) -> Result<Self, Error> {
        Ok(Self {
            class: ElementClass::Literal,
            data: read_file(path, what_for)?.into(),
        })
    }

    fn load(&mut self, base: &(String, PathBuf), what_for: &'static str) -> Result<(), Error> {
        if self.class == ElementClass::File {
            self.data = read_file(&(format!("{}{}{}",
                                            base.0,
                                            if !is_path_separator(base.0.as_bytes()[base.0.as_bytes().len() - 1] as char) &&
                                               !is_path_separator(self.data.as_bytes()[0] as char) {
                                                "/"
                                            } else {
                                                ""
                                            },
                                            self.data),
                                    concat_path(base.1.clone(), &self.data)),
                                  what_for)
                ?
                .into();
            self.class = ElementClass::Literal;
        }

        Ok(())
    }
}

impl<Dt: WrappedElementImplData> WrappedElement for WrappedElementImpl<Dt> {
    fn head(&self) -> &str {
        match self.0.class {
            ElementClass::Link => Dt::link_head(),
            ElementClass::Literal => Dt::literal_head(),
            ElementClass::File => "&lt;",
        }
    }

    fn content(&self) -> &str {
        &self.0.data
    }

    fn foot(&self) -> &str {
        match self.0.class {
            ElementClass::Link => Dt::link_foot(),
            ElementClass::Literal => Dt::literal_foot(),
            ElementClass::File => "&gt;\n",
        }
    }
}


const ELEMENT_FIELDS: &[&str] = &["class", "data"];

struct WrappedElementImplVisitor<Dt: WrappedElementImplData>(PhantomData<Dt>);

impl<'de, Dt: WrappedElementImplData> de::Visitor<'de> for WrappedElementImplVisitor<Dt> {
    type Value = WrappedElementImpl<Dt>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct ")?;
        formatter.write_str(Dt::user_facing_name())?;
        Ok(())
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        wrapped_element_impl_visitor_visit_str(v).map(|dt| WrappedElementImpl(dt, PhantomData))
    }

    fn visit_map<V: de::MapAccess<'de>>(self, map: V) -> Result<Self::Value, V::Error> {
        wrapped_element_impl_visitor_visit_map(map).map(|dt| WrappedElementImpl(dt, PhantomData))
    }
}

fn wrapped_element_impl_visitor_visit_str<E: de::Error>(v: &str) -> Result<WrappedElementImplImpl, E> {
    let mut itr = v.splitn(2, ":");
    Ok(match (itr.next(), itr.next()) {
        (Some(val), None) |
        (Some("literal"), Some(val)) => {
            WrappedElementImplImpl {
                class: ElementClass::Literal,
                data: val.to_string().into(),
            }
        }
        (Some("link"), Some(val)) => {
            WrappedElementImplImpl {
                class: ElementClass::Link,
                data: val.to_string().into(),
            }
        }
        (Some("file"), Some(val)) => {
            WrappedElementImplImpl {
                class: ElementClass::File,
                data: val.to_string().into(),
            }
        }

        (Some(tp), Some(_)) => return Err(de::Error::invalid_value(de::Unexpected::Str(tp), &r#""literal", "link", or "file""#)),
        (None, ..) => unreachable!(),
    })
}

fn wrapped_element_impl_visitor_visit_map<'de, V: de::MapAccess<'de>>(mut map: V) -> Result<WrappedElementImplImpl, V::Error> {
    let mut class = None;
    let mut data = None;
    while let Some(key) = map.next_key()? {
        match key {
            "class" => {
                if class.is_some() {
                    return Err(de::Error::duplicate_field("class"));
                }
                class = Some(match map.next_value()? {
                    "literal" => ElementClass::Literal,
                    "link" => ElementClass::Link,
                    "file" => ElementClass::File,
                    val => return Err(de::Error::invalid_value(de::Unexpected::Str(val), &r#""literal", "link", or "file""#)),
                });
            }
            "data" => {
                if data.is_some() {
                    return Err(de::Error::duplicate_field("data"));
                }
                data = Some(map.next_value()?);
            }
            _ => return Err(de::Error::unknown_field(key, ELEMENT_FIELDS)),
        }
    }

    Ok(WrappedElementImplImpl {
        class: class.ok_or_else(|| de::Error::missing_field("class"))?,
        data: data.ok_or_else(|| de::Error::missing_field("data"))?,
    })
}

impl<'de, Dt: WrappedElementImplData> de::Deserialize<'de> for WrappedElementImpl<Dt> {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_struct(Dt::user_facing_name(), ELEMENT_FIELDS, WrappedElementImplVisitor(PhantomData))
    }
}
