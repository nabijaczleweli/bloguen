#[cfg(not(target_os = "windows"))]
extern crate gcc;

#[cfg(not(target_os = "windows"))]
use std::env;
#[cfg(not(target_os = "windows"))]
use std::io::Write;
#[cfg(not(target_os = "windows"))]
use std::path::Path;
#[cfg(not(target_os = "windows"))]
use std::fs::{self, File};

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

    get_errno_data();
}

#[cfg(target_os = "windows")]
fn get_errno_data() {}

#[cfg(not(target_os = "windows"))]
fn get_errno_data() {
    let errno_dir = Path::new(&env::var("OUT_DIR").unwrap()).join("errno-data");
    fs::create_dir_all(&errno_dir).unwrap();

    let errno_source = errno_dir.join("errno.c");
    File::create(&errno_source).unwrap().write_all(ERANGE_CHECK_SOURCE.as_bytes()).unwrap();

    let errno_preprocessed = String::from_utf8(gcc::Build::new().file(errno_source).expand()).unwrap();
    let errno_expr = errno_preprocessed.lines().next_back().unwrap();

    let errno_include = errno_dir.join("errno.rs");
    File::create(&errno_include).unwrap().write_all(ERANGE_INCLUDE_SKELETON.replace("{}", &errno_expr).as_bytes()).unwrap();
}
