extern crate reqwest;
#[cfg(not(target_os = "windows"))]
extern crate cc;

use std::io::{BufReader, BufRead, Write};
use std::collections::BTreeSet;
use std::path::Path;
use std::fs::File;
use std::env;
#[cfg(not(target_os = "windows"))]
use std::fs;

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
    get_errno_data(&out_dir);
    words(&out_dir);
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

fn words(out_dir: &Path) {
    let dest_path = out_dir.join("words.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all("/// A set of upper-case-first adjectives for random string gen.\n".as_bytes()).unwrap();
    f.write_all("pub static ADJECTIVES: &[&str] = &[\n".as_bytes()).unwrap();
    for adj in words_first_adjectives().into_iter().chain(words_second_adjectives().into_iter()).map(uppercase_first).collect::<BTreeSet<_>>() {
        f.write_all("   \"".as_bytes()).unwrap();
        f.write_all(adj.as_bytes()).unwrap();
        f.write_all("\",\n".as_bytes()).unwrap();
    }
    f.write_all("];\n".as_bytes()).unwrap();
    f.write_all("\n".as_bytes()).unwrap();
    f.write_all("/// A set of upper-case-first nouns for random string gen.\n".as_bytes()).unwrap();
    f.write_all("pub static NOUNS: &[&str] = &[\n".as_bytes()).unwrap();
    for noun in words_nouns() {
        f.write_all("   \"".as_bytes()).unwrap();
        f.write_all(uppercase_first(noun).as_bytes()).unwrap();
        f.write_all("\",\n".as_bytes()).unwrap();
    }
    f.write_all("];\n".as_bytes()).unwrap();
    f.write_all("\n".as_bytes()).unwrap();
    f.write_all("/// A set of upper-case-first adverbs for random string gen.\n".as_bytes()).unwrap();
    f.write_all("pub static ADVERBS: &[&str] = &[\n".as_bytes()).unwrap();
    for adv in words_first_adverbs().into_iter().chain(words_second_adverbs().into_iter()).map(uppercase_first).collect::<BTreeSet<_>>() {
        f.write_all("   \"".as_bytes()).unwrap();
        f.write_all(uppercase_first(adv).as_bytes()).unwrap();
        f.write_all("\",\n".as_bytes()).unwrap();
    }
    f.write_all("];\n".as_bytes()).unwrap();
}

/// Stolen from https://stackoverflow.com/a/38406885/2851815.
fn uppercase_first(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn words_first_adjectives() -> Vec<String> {
    let mut currently = false;
    let mut coll = vec![];

    for l in BufReader::new(reqwest::Client::builder()
            .gzip(true)
            .build()
            .unwrap()
            .get("http://enchantedlearning.com/wordlist/adjectives.shtml")
            .send()
            .unwrap())
        .lines() {
        let l = l.unwrap();
        for l in l.split("\r") {
            let l = l.to_lowercase();

            if l == "</td></tr></table>" {
                currently = false;
            }

            if currently && !l.is_empty() {
                let l = l.replace("<br>", "");
                if !l.contains('<') && l.len() > 1 {
                    coll.push(l);
                }
            }

            if l.contains("<font size=+0>a</font>") {
                currently = true;
                continue;
            }
        }
    }

    coll
}

fn words_second_adjectives() -> Vec<String> {
    words_talkenglish("http://www.talkenglish.com/vocabulary/top-1500-nouns.aspx")
}

fn words_nouns() -> Vec<String> {
    BufReader::new(reqwest::Client::builder()
            .gzip(true)
            .build()
            .unwrap()
            .get("http://www.desiquintans.com/downloads/nounlist/nounlist.txt")
            .send()
            .unwrap())
        .lines()
        .map(Result::unwrap)
        .filter(|l| l.len() != 1)
        .collect()
}

fn words_first_adverbs() -> Vec<String> {
    words_talkenglish("http://www.talkenglish.com/vocabulary/top-250-adverbs.aspx")
}

fn words_second_adverbs() -> Vec<String> {
    let mut currently = false;
    let mut coll = vec![];

    for l in BufReader::new(reqwest::Client::builder()
            .gzip(true)
            .build()
            .unwrap()
            .get("https://www.espressoenglish.net/100-common-english-adverbs/")
            .send()
            .unwrap())
        .lines() {
        let l = l.unwrap();
        for l in l.split("\r") {
            let l = l.to_lowercase();

            if l.contains("100.") {
                currently = false;
            }

            if currently && !l.contains("div>") {
                let l = l.replace("<br />", "").replace("<p>", "").replace("</p>", "").replace("/>", "");
                coll.push(l.rsplitn(2, " ").next().unwrap().trim().to_string());
            }

            if l.contains("<p>1.") {
                currently = true;
                continue;
            }
        }
    }

    coll
}

fn words_talkenglish(url: &str) -> Vec<String> {
    BufReader::new(reqwest::Client::builder()
            .gzip(true)
            .build()
            .unwrap()
            .get(url)
            .send()
            .unwrap())
        .lines()
        .map(Result::unwrap)
        .filter(|l| l.contains(r#"<a href="/how-to-use/"#))
        .filter_map(|l| l.replace("</a>", "").replace('>', "\n").split("\n").skip(1).next().map(|s| s.to_string()))
        .skip(1)
        .filter(|l| l.len() != 1)
        .collect()
}
