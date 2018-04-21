//! Generate an ePub book from a simple plaintext descriptor
//!
//! # Library doc
//!
//! This library is used by `bloguen` itself for all its function and is therefore contains all necessary functions.
//!
//! ## Data flow
//!
//! ```text
//! Options
//! |> parse_descriptor()
//! |> EPubBook::from_elements()
//! |> EPubBook::normalise_paths()
//! |> EPubBook::write_zip()
//! ```


#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate walkdir;
extern crate chrono;
extern crate comrak;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate clap;
extern crate toml;
extern crate crc;

pub mod ops;
pub mod util;

mod error;
mod options;

pub use error::Error;
pub use options::Options;
