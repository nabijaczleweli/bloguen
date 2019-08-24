//! Generate an ePub book from a simple plaintext descriptor
//!
//! See the [`bloguen` executable manpage](https://rawcdn.githack.com/nabijaczleweli/bloguen/man/bloguen.1.html),
//! or the [`ops`](ops/) module for library doc.
//!
//! # Special thanks
//!
//! To all who support further development on [Patreon](https://patreon.com/nabijaczleweli), in particular:
//!
//!   * ThePhD


extern crate percent_encoding;
extern crate safe_transmute;
extern crate rand_xorshift;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bidir_map;
extern crate jetscii;
extern crate unicase;
extern crate walkdir;
extern crate chrono;
extern crate comrak;
#[cfg(target_os = "windows")]
extern crate winapi;
extern crate regex;
extern crate serde;
extern crate rand;
#[macro_use]
extern crate clap;
#[cfg(not(target_os = "windows"))]
extern crate libc;
extern crate toml;
extern crate uuid;
extern crate crc;
extern crate url;

pub mod ops;
pub mod util;

mod error;
mod options;

pub use error::Error;
pub use options::Options;
