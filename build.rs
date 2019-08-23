#[cfg(not(target_os = "windows"))]
extern crate cc;

use std::fs::{self, File};
use std::path::Path;
use std::io::Write;
use std::env;


/// The last line of this, after running it through a preprocessor, will expand to the value of `ERANGE`
#[cfg(not(target_os = "windows"))]
static ERANGE_CHECK_SOURCE: &str = r#"
#include <errno.h>

ERANGE
"#;

/// Replace `{}` with the `ERANGE` expression from `ERANGE_CHECK_SOURCE`
#[cfg(not(target_os = "windows"))]
static ERANGE_INCLUDE_SKELETON: &str = r#"
/// Value of `ERANGE` from `errno.h`
const ERANGE: c_int = {};
"#;


fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    simple_parsable("center_order",
                    "CenterOrder",
                    "center order",
                    "A specifier of index centerpiece ordering.",
                    &[("Forward", "forward", "Low-to-high", "forwaRd"), ("Backward", "backward", "High-to-low", "BaCkWard")],
                    &out_dir);

    simple_parsable("machine_data",
                    "MachineDataKind",
                    "machine data specifier",
                    "A specifier of machine data format.",
                    &[("Json", "JSON", "JSON with all the data except content in it", "JsoN")],
                    &out_dir);

    simple_parsable("feed_type",
                    "FeedType",
                    "feed type",
                    "A feed type specifier.",
                    &[("Rss", "RSS", "[RSS 2.0](https://en.wikipedia.org/wiki/RSS)", "RsS"),
                      ("Atom", "Atom", "[Atom](https://en.wikipedia.org/wiki/Atom_(Web_standard))", "atOM")],
                    &out_dir);


    wrapped_element("Script",
                    ("/content/assets/syllable.js", "document.getElementById(\\\"title\\\").innerText = \\\"Наган\\\";", "MathJax-config.js"),
                    "<script type=\"text/javascript\" src=\"/content/assets/syllable.js\"></script>\n",
                    "<script type=\"text/javascript\">\n\ndocument.getElementById(\"title\").innerText = \"Наган\";\n\n</script>\n",
                    ("MathJax-config",
                     "js",
                     r#"MathJax.Hub.Config({
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
    /// });"#,
                     r#"
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
    /// #     });\n\"#,
                     r#""<script type=\"text/javascript\">\n\n\
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
    /// \n\n</script>\n""#,
                     r#"
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
    ///     });\n\"#),
                    ("octicons", r#"window.addEventListener("load", function() {
    ///     const PLACEHOLDER = document.getElementById("octicons-placeholder");
    ///
    ///     const request = new XMLHttpRequest();
    ///     request.open("GET", "/content/assets/octicons/sprite.octicons.svg");
    ///     request.onload = function(load) {
    ///         PLACEHOLDER.outerHTML = load.target.responseText.replace("<svg", "<svg class=\"hidden\"");
    ///     };
    ///     request.send();
    /// });"#, r#"window.addEventListener(\"load\", function() {\n\
    /// #     const PLACEHOLDER = document.getElementById(\"octicons-placeholder\");\n\
    /// #     \n\
    /// #     const request = new XMLHttpRequest();\n\
    /// #     request.open(\"GET\", \"/content/assets/octicons/sprite.octicons.svg\");\n\
    /// #     request.onload = function(load) {\n\
    /// #         PLACEHOLDER.outerHTML = load.target.responseText.replace(\"<svg\", \"<svg class=\\\"hidden\\\"\");\n\
    /// #     };\n\
    /// #     request.send();\n\
    /// # });\n\"#, r#"
    /// window.addEventListener(\"load\", function() {\n\
    ///     const PLACEHOLDER = document.getElementById(\"octicons-placeholder\");\n\
    ///     \n\
    ///     const request = new XMLHttpRequest();\n\
    ///     request.open(\"GET\", \"/content/assets/octicons/sprite.octicons.svg\");\n\
    ///     request.onload = function(load) {\n\
    ///         PLACEHOLDER.outerHTML = load.target.responseText.replace(\"<svg\", \"<svg class=\\\"hidden\\\"\");\n\
    ///     };\n\
    ///     request.send();\n\
    /// });\n\"#),
                    &out_dir);

    wrapped_element("Style",
                    ("//nabijaczleweli.xyz/kaschism/assets/column.css", ".indented { text-indent: 1em; }", "common.css"),
                    "<link href=\"//nabijaczleweli.xyz/kaschism/assets/column.css\" rel=\"stylesheet\" />\n",
                    "<style type=\"text/css\">\n\n.indented { text-indent: 1em; }\n\n</style>\n",
                    ("common",
                     "css",
                     r#"ul, ol {
    ///     margin-top: 0;
    ///     margin-bottom: 0;
    /// }
    ///
    /// a > i.fa {
    ///     color: black;
    /// }"#,
                     r#"
    /// #     ul, ol {\n\
    /// #         margin-top: 0;\n\
    /// #         margin-bottom: 0;\n\
    /// #     }\n\
    /// #     \n\
    /// #     a > i.fa {\n\
    /// #         color: black;\n\
    /// #     }\n"#,
                     r#""<style type=\"text/css\">\n\n\
    ///      ul, ol {\n\
    ///          margin-top: 0;\n\
    ///          margin-bottom: 0;\n\
    ///      }\n\
    ///      \n\
    ///      a > i.fa {\n\
    ///          color: black;\n\
    ///      }\n\n\
    /// \n\n</style>\n""#,
                     r#"
    ///     ul, ol {\n\
    ///         margin-top: 0;\n\
    ///         margin-bottom: 0;\n\
    ///     }\n\
    ///     \n\
    ///     a > i.fa {\n\
    ///         color: black;\n\
    ///     }\n"#),
                    ("effects",
                     r#".ruby {
    ///     /* That's Ruby according to https://en.wikipedia.org/wiki/Ruby_(color). */
    ///     color: #E0115F;
    /// }"#,
                     r#".ruby {\n\
    /// #     /* That's Ruby according to https://en.wikipedia.org/wiki/Ruby_(color). */\n\
    /// #     color: #E0115F;\n\
    /// # }\n"#,
                     r#"
    ///    .ruby {\n\
    ///         /* That's Ruby according to https://en.wikipedia.org/wiki/Ruby_(color). */\n\
    ///         color: #E0115F;\n\
    ///     }\n"#),
                    &out_dir);


    get_errno_data(&out_dir);
}


fn simple_parsable(mod_name: &str, type_name: &str, type_human_name: &str, type_doc: &str, elements: &[(&str, &str, &str, &str)], out_dir: &Path) {
    let parsables_dir = out_dir.join("simple-parsable");
    fs::create_dir_all(&parsables_dir).unwrap();

    let mut out = File::create(parsables_dir.join(format!("{}.rs", mod_name))).unwrap();

    write!(&mut out,
           r#"
mod {} {{


use serde::de::{{Deserializer, Deserialize, Error as SerdeError}};
use serde::ser::{{Serializer, Serialize}};
use self::super::super::Error;
use bidir_map::BidirMap;
use std::str::FromStr;
use unicase::UniCase;
use std::fmt;


lazy_static! {{
    static ref NAME_ORDER_MAP: BidirMap<UniCase<&'static str>, {}> = bidir_map!{{
"#,
           mod_name,
           type_name)
        .unwrap();

    for (el_variant_name, el_canon_name, _, _) in elements {
        writeln!(&mut out, r#"        UniCase::new({:?}) => {}::{},"#, el_canon_name, type_name, el_variant_name).unwrap();
    }

    write!(&mut out,
           r#"    }};

    static ref ERROR_WHER: String = String::from_utf8(NAME_ORDER_MAP.first_col()
            .enumerate()
            .map(|(i, v)| (i == NAME_ORDER_MAP.len() - 1, v))
            .fold((true, "expected ".as_bytes().to_vec()), |(first, mut acc), (last, el)| {{
                if !first {{
                    if NAME_ORDER_MAP.len() != 2 {{
                        acc.extend(b",");
                    }}
                    acc.extend(b" ");
                    if last {{
                        acc.extend(b"or ");
                    }}
                }}

                acc.extend(b"\"");
                acc.extend(el.as_bytes());
                acc.extend(b"\"");

                (false, acc)
            }})
            .1)
        .unwrap();
}}


/// {}
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum {} {{
"#,
           type_doc,
           type_name)
        .unwrap();

    for (el_variant_name, _, el_doc, _) in elements {
        writeln!(&mut out, r#"    /// {}"#, el_doc).unwrap();
        writeln!(&mut out, r#"    {},"#, el_variant_name).unwrap();
    }

    write!(&mut out,
           r#"}}

impl {} {{
    /// Get a {} corresponding to the specified string.
    ///
    /// The string repr of any variant is its name, case-insensitive.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::{0};
"#,
           type_name,
           type_human_name)
        .unwrap();

    for (el_variant_name, _, _, el_mangled_name) in elements {
        writeln!(&mut out,
                 r#"    /// assert_eq!({}::from({:?}), Some({0}::{}));"#,
                 type_name,
                 el_mangled_name,
                 el_variant_name)
            .unwrap();
    }

    write!(&mut out,
           r#"    /// ```
    pub fn from(s: &str) -> Option<{}> {{
        NAME_ORDER_MAP.get_by_first(&UniCase::new(s)).map(|&k| k)
    }}

    /// Get a human-readable name of this {}.
    ///
    /// This is re-`from()`able to self.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::{0};
"#,
           type_name,
           type_human_name)
        .unwrap();

    for (el_variant_name, el_canon_name, _, _) in elements {
        writeln!(&mut out,
                 r#"    /// assert_eq!({}::{}.name(), {:?});"#,
                 type_name,
                 el_variant_name,
                 el_canon_name)
            .unwrap();
    }

    write!(&mut out,
           r#"    /// ```
    pub fn name(&self) -> &'static str {{
        NAME_ORDER_MAP.get_by_second(&self).unwrap()
    }}
}}

impl FromStr for {} {{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        {0}::from(s).ok_or_else(|| {{
            Error::Parse {{
                tp: {:?},
                wher: ERROR_WHER[..].into(),
                more: format!("\"{{}}\" invalid", s).into(),
            }}
        }})
    }}
}}

impl<'de> Deserialize<'de> for {0} {{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {{
        {0}::from_str(<&'de str>::deserialize(deserializer)?).map_err(|e| {{
            let buf = e.to_string();
            D::Error::custom(&buf[..buf.len() - 1]) // Drop dot
        }})
    }}
}}

impl Serialize for {0} {{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {{
        serializer.serialize_str(self.name())
    }}
}}

impl fmt::Display for {0} {{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{
        self.name().fmt(f)
    }}
}}


}}
"#,
           type_name,
           type_human_name)
        .unwrap();
}


fn wrapped_element(elem_name: &str, llp: (&str, &str, &str), link_printed: &str, literal_printed: &str, file: (&str, &str, &str, &str, &str, &str),
                   second_file: (&str, &str, &str, &str), out_dir: &Path) {
    let mod_name = elem_name.to_lowercase();
    let cap_name = elem_name.to_uppercase();

    let wrappeds_dir = out_dir.join("wrapped_element");
    fs::create_dir_all(&wrappeds_dir).unwrap();

    let mut out = File::create(wrappeds_dir.join(format!("{}.rs", mod_name))).unwrap();

    write!(&mut out,
           r###"
mod {mod_name}_element {{


use self::super::super::super::super::util::{{concat_path, read_file}};
use std::path::{{PathBuf, is_separator as is_path_separator}};
use self::super::{{WrappedElement, ElementClass}};
use self::super::super::super::super::Error;
use std::borrow::Cow;
use serde::de;
use std::fmt;


lazy_static! {{
    static ref {cap_name}_LINK_HEAD: &'static str = include_str!("../../../../../../assets/element_wrappers/{mod_name}/link.head").trim();
    static ref {cap_name}_LINK_FOOT: &'static str = include_str!("../../../../../../assets/element_wrappers/{mod_name}/link.foot").trim_start();

    static ref {cap_name}_LITERAL_HEAD: &'static str = include_str!("../../../../../../assets/element_wrappers/{mod_name}/literal.head").trim_start();
    static ref {cap_name}_LITERAL_FOOT: &'static str = include_str!("../../../../../../assets/element_wrappers/{mod_name}/literal.foot");
}}


/// A {mod_name} specifier.
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
/// # use bloguen::ops::{elem_name}Element;
/// #[derive(Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// struct {elem_name}Container {{
///     pub {mod_name}: Vec<{elem_name}Element>,
/// }}
///
/// # fn main() {{
/// let {mod_name}_toml =
///     "[[{mod_name}]]
///      class = 'link'
///      data = '{llp_0}'
///
///      [[{mod_name}]]
///      class = 'literal'
///      data = '{llp_1}'
///
///      [[{mod_name}]]
///      class = 'file'
///      data = '{llp_2}'";
///
/// let {elem_name}Container {{ {mod_name} }} = toml::from_str({mod_name}_toml).unwrap();
/// assert_eq!(&{mod_name},
///            &[{elem_name}Element::from_link("{llp_0}"),
///              {elem_name}Element::from_literal("{llp_1}"),
///              {elem_name}Element::from_path("{llp_2}")]);
/// # }}
/// ```
///
/// And a compact one (the "literal" tag may be omitted if the content doesn't contain any colons):
///
/// ```
/// # extern crate toml;
/// # extern crate bloguen;
/// # #[macro_use]
/// # extern crate serde_derive;
/// # use bloguen::ops::{elem_name}Element;
/// #[derive(Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// struct {elem_name}Container {{
///     pub {mod_name}s: Vec<{elem_name}Element>,
/// }}
///
/// # fn main() {{
/// let {mod_name}s_toml =
///     "{mod_name}s = [
///          'link:{llp_0}',
///          'literal:{llp_1}',
///          'file:{llp_2}',
///      ]";
///
/// let {elem_name}Container {{ {mod_name}s }} = toml::from_str({mod_name}s_toml).unwrap();
/// assert_eq!(&{mod_name}s,
///            &[{elem_name}Element::from_link("{llp_0}"),
///              {elem_name}Element::from_literal("{llp_1}"),
///              {elem_name}Element::from_path("{llp_2}")]);
/// # }}
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct {elem_name}Element {{
    class: ElementClass,
    data: Cow<'static, str>,
}}

impl {elem_name}Element {{
    /// Create a {mod_name} element linking to an external resource.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::{{WrappedElement, {elem_name}Element}};
    /// let lonk = {elem_name}Element::from_link("{llp_0}");
    /// assert_eq!(
    ///     format!("{{}}{{}}{{}}", lonk.head(), lonk.content(), lonk.foot()),
    ///     {link_printed:?})
    /// ```
    pub fn from_link<Dt: Into<Cow<'static, str>>>(link: Dt) -> {elem_name}Element {{
        {elem_name}Element::from_link_impl(link.into())
    }}

    fn from_link_impl(link: Cow<'static, str>) -> {elem_name}Element {{
        {elem_name}Element {{
            class: ElementClass::Link,
            data: link,
        }}
    }}

    /// Create a {mod_name} element including the specified literal literally.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::ops::{{WrappedElement, {elem_name}Element}};
    /// let lit = {elem_name}Element::from_literal("{llp_1}");
    /// assert_eq!(
    ///     format!("{{}}{{}}{{}}", lit.head(), lit.content(), lit.foot()),
    ///     {literal_printed:?})
    /// ```
    pub fn from_literal<Dt: Into<Cow<'static, str>>>(literal: Dt) -> {elem_name}Element {{
        {elem_name}Element::from_literal_impl(literal.into())
    }}

    fn from_literal_impl(literal: Cow<'static, str>) -> {elem_name}Element {{
        {elem_name}Element {{
            class: ElementClass::Literal,
            data: literal,
        }}
    }}

    /// Create a {mod_name} element pointing to the specified relative path.
    ///
    /// Consult [`load()`](#fn.load) documentation for more data.
    ///
    /// # Examples
    ///
    /// Given `$ROOT/{file_name}.{file_extension}` containing:
    ///
    /// ```{file_extension}
    /// {file_doc}
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::{{WrappedElement, {elem_name}Element}};
    /// # use std::fs::{{self, File}};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # use bloguen::Error;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-output-wrapped_element-{mod_name}_element-from_path");
    /// # fs::create_dir_all(&root).unwrap();
    /// # File::create(root.join("{file_name}.{file_extension}")).unwrap().write_all("\{file_written}
    /// # ".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    ///
    /// let mut lit_p = {elem_name}Element::from_path("{file_name}.{file_extension}");
    /// assert_eq!(lit_p.load(&("$ROOT".to_string(), root.clone())), Ok(()));
    /// assert_eq!(format!("{{}}{{}}{{}}", lit_p.head(), lit_p.content(), lit_p.foot()),
    /// {file_formatted});
    /// ```
    pub fn from_path<Dt: Into<Cow<'static, str>>>(path: Dt) -> {elem_name}Element {{
        {elem_name}Element::from_path_impl(path.into())
    }}

    fn from_path_impl(path: Cow<'static, str>) -> {elem_name}Element {{
        {elem_name}Element {{
            class: ElementClass::File,
            data: path.into(),
        }}
    }}

    /// Create a literal {mod_name} element from the contents of the specified file.
    ///
    /// # Examples
    ///
    /// Given `$ROOT/{file_name}.{file_extension}` containing:
    ///
    /// ```{file_extension}
    /// {file_doc}
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::{{WrappedElement, {elem_name}Element}};
    /// # use std::fs::{{self, File}};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # use bloguen::Error;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-output-wrapped_element-{mod_name}_element-from_file");
    /// # fs::create_dir_all(&root).unwrap();
    /// # File::create(root.join("{file_name}.{file_extension}")).unwrap().write_all("\{file_written}
    /// # ".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    ///
    /// let lit_p = {elem_name}Element::from_file(&("$ROOT/{file_name}.{file_extension}".to_string(), root.join("{file_name}.{file_extension}"))).unwrap();
    /// assert_eq!(format!("{{}}{{}}{{}}", lit_p.head(), lit_p.content(), lit_p.foot()),
    /// {file_formatted});
    /// ```
    pub fn from_file(path: &(String, PathBuf)) -> Result<{elem_name}Element, Error> {{
        Ok({elem_name}Element {{
            class: ElementClass::Literal,
            data: read_file(path, "literal {mod_name} element from path")?.into(),
        }})
    }}

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
    ///   {file_name}.{file_extension}
    ///   assets
    ///     {second_filename}.{file_extension}
    /// ```
    ///
    /// Given `$ROOT/{file_name}.{file_extension}` containing:
    ///
    /// ```{file_extension}
    /// {file_doc}
    /// ```
    ///
    /// Given `$ROOT/assets/{second_filename}.{file_extension}` containing:
    ///
    /// ```{file_extension}
    /// {second_filedoc}
    /// ```
    ///
    /// The following holds:
    ///
    /// ```
    /// # use bloguen::ops::{elem_name}Element;
    /// # use std::fs::{{self, File}};
    /// # use std::env::temp_dir;
    /// # use std::io::Write;
    /// # use bloguen::Error;
    /// # let root = temp_dir().join("bloguen-doctest").join("ops-output-wrapped_element-{mod_name}_element-load");
    /// # fs::create_dir_all(root.join("assets")).unwrap();
    /// # File::create(root.join("{file_name}.{file_extension}")).unwrap().write_all("\{file_written}
    /// # ".as_bytes()).unwrap();
    /// # File::create(root.join("assets").join("{second_filename}.{file_extension}")).unwrap().write_all("{second_filewritten}
    /// # ".as_bytes()).unwrap();
    /// # /*
    /// let root: PathBuf = /* obtained elsewhere */;
    /// # */
    ///
    /// let mut elem = {elem_name}Element::from_path("{file_name}.{file_extension}");
    /// assert_eq!(elem.load(&("$ROOT".to_string(), root.clone())), Ok(()));
    /// assert_eq!(elem, {elem_name}Element::from_literal("\{file_contents}
    /// "));
    ///
    /// let mut elem = {elem_name}Element::from_path("assets/.././assets/{second_filename}.{file_extension}");
    /// assert_eq!(elem.load(&("$ROOT".to_string(), root.clone())), Ok(()));
    /// assert_eq!(elem, {elem_name}Element::from_literal("\{second_fileformatted}
    /// "));
    ///
    /// let mut elem = {elem_name}Element::from_path("assets/nonexistant.{file_extension}");
    /// assert_eq!(elem.load(&("$ROOT".to_string(), root.clone())), Err(Error::FileNotFound {{
    ///     who: "file {mod_name} element",
    ///     path: "$ROOT/assets/nonexistant.{file_extension}".into(),
    /// }}));
    /// assert_eq!(elem, {elem_name}Element::from_path("assets/nonexistant.{file_extension}"));
    /// ```
    pub fn load(&mut self, base: &(String, PathBuf)) -> Result<(), Error> {{
        if self.class == ElementClass::File {{
            self.data = read_file(&(format!("{{}}{{}}{{}}",
                                            base.0,
                                            if !is_path_separator(base.0.as_bytes()[base.0.as_bytes().len() - 1] as char) &&
                                               !is_path_separator(self.data.as_bytes()[0] as char) {{
                                                "/"
                                            }} else {{
                                                ""
                                            }},
                                            self.data),
                                    concat_path(base.1.clone(), &self.data)),
                                  "file {mod_name} element")
                ?
                .into();
            self.class = ElementClass::Literal;
        }}

        Ok(())
    }}
}}

impl WrappedElement for {elem_name}Element {{
    fn head(&self) -> &str {{
        match self.class {{
            ElementClass::Link => &{cap_name}_LINK_HEAD,
            ElementClass::Literal => &{cap_name}_LITERAL_HEAD,
            ElementClass::File => "&lt;",
        }}
    }}

    fn content(&self) -> &str {{
        &self.data
    }}

    fn foot(&self) -> &str {{
        match self.class {{
            ElementClass::Link => &{cap_name}_LINK_FOOT,
            ElementClass::Literal => &{cap_name}_LITERAL_FOOT,
            ElementClass::File => "&gt;\n",
        }}
    }}
}}


const {cap_name}_FIELDS: &[&str] = &["class", "data"];

struct {elem_name}ElementVisitor;

impl<'de> de::Visitor<'de> for {elem_name}ElementVisitor {{
    type Value = {elem_name}Element;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {{
        formatter.write_str("struct {elem_name}Element")
    }}

    fn visit_str<E: de::Error>(self, v: &str) -> Result<{elem_name}Element, E> {{
        let mut itr = v.splitn(2, ":");
        Ok(match (itr.next(), itr.next()) {{
            (Some(val), None) |
            (Some("literal"), Some(val)) => {{
                {elem_name}Element {{
                    class: ElementClass::Literal,
                    data: val.to_string().into(),
                }}
            }}
            (Some("link"), Some(val)) => {{
                {elem_name}Element {{
                    class: ElementClass::Link,
                    data: val.to_string().into(),
                }}
            }}
            (Some("file"), Some(val)) => {{
                {elem_name}Element {{
                    class: ElementClass::File,
                    data: val.to_string().into(),
                }}
            }}

            (Some(tp), Some(_)) => return Err(de::Error::invalid_value(de::Unexpected::Str(tp), &r#""literal", "link", or "file""#)),
            (None, ..) => unreachable!(),
        }})
    }}

    fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<{elem_name}Element, V::Error> {{
        let mut class = None;
        let mut data = None;
        while let Some(key) = map.next_key()? {{
            match key {{
                "class" => {{
                    if class.is_some() {{
                        return Err(de::Error::duplicate_field("class"));
                    }}
                    class = Some(match map.next_value()? {{
                        "literal" => ElementClass::Literal,
                        "link" => ElementClass::Link,
                        "file" => ElementClass::File,
                        val => return Err(de::Error::invalid_value(de::Unexpected::Str(val), &r#""literal", "link", or "file""#)),
                    }});
                }}
                "data" => {{
                    if data.is_some() {{
                        return Err(de::Error::duplicate_field("data"));
                    }}
                    data = Some(map.next_value()?);
                }}
                _ => return Err(de::Error::unknown_field(key, {cap_name}_FIELDS)),
            }}
        }}

        Ok({elem_name}Element {{
            class: class.ok_or_else(|| de::Error::missing_field("class"))?,
            data: data.ok_or_else(|| de::Error::missing_field("data"))?,
        }})
    }}
}}

impl<'de> de::Deserialize<'de> for {elem_name}Element {{
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {{
        deserializer.deserialize_struct("{elem_name}Element", {cap_name}_FIELDS, {elem_name}ElementVisitor)
    }}
}}


}}
"###,
           mod_name = mod_name,
           elem_name = elem_name,
           cap_name = cap_name,

           llp_0 = llp.0,
           llp_1 = llp.1,
           llp_2 = llp.2,

           link_printed = link_printed,
           literal_printed = literal_printed,

           file_name = file.0,
           file_extension = file.1,
           file_doc = file.2,
           file_written = file.3,
           file_formatted = file.4,
           file_contents = file.5,

           second_filename = second_file.0,
           second_filedoc = second_file.1,
           second_filewritten = second_file.2,
           second_fileformatted = second_file.3)
        .unwrap();
}

#[cfg(target_os = "windows")]
fn get_errno_data(_: &Path) {}

#[cfg(not(target_os = "windows"))]
fn get_errno_data(out_dir: &Path) {
    let errno_dir = out_dir.join("errno-data");
    fs::create_dir_all(&errno_dir).unwrap();

    let errno_source = errno_dir.join("errno.c");
    File::create(&errno_source).unwrap().write_all(ERANGE_CHECK_SOURCE.as_bytes()).unwrap();

    let errno_preprocessed = String::from_utf8(cc::Build::new().file(errno_source).expand()).unwrap();
    let errno_expr = errno_preprocessed.lines().next_back().unwrap();

    let errno_include = errno_dir.join("errno.rs");
    File::create(&errno_include).unwrap().write_all(ERANGE_INCLUDE_SKELETON.replace("{}", &errno_expr).as_bytes()).unwrap();
}
