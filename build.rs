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
