//! This module contains the configuration of the application.
//!
//! All options are passed individually to each function and are not bundled together.
//!
//! # Examples
//!
//! ```no_run
//! # use bloguen::Options;
//! let options = Options::parse();
//! println!("Generating blogue from {} to {}", options.source_dir.0, options.output_dir.0);
//! ```


use clap::{ErrorKind as ClapErrorKind, Error as ClapError, AppSettings, Arg};
use std::path::{PathBuf, Path};
use std::fs;


/// Representation of the application's all configurable values.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Options {
    /// The directory containing the blogue source, must exist.
    pub source_dir: (String, PathBuf),
    /// The directory to the blogue source, must not exist if `--force` not specified, parent directory must exist.
    pub output_dir: (String, PathBuf),
}

impl Options {
    /// Parse `env`-wide command-line arguments into an `Options` instance
    pub fn parse() -> Options {
        let matches = app_from_crate!("\n")
            .setting(AppSettings::ColoredHelp)
            .arg(Arg::from_usage("<IN_DIR> 'Directory to generate a blogue from'").validator(Options::source_directory_validator))
            .arg(Arg::from_usage("<OUT_DIR> 'Directory to generate the blogue into'").validator(Options::output_directory_validator))
            .arg(Arg::from_usage("-f --force 'Allow the output directory to exist, overriding it'"))
            .get_matches();

        Options {
            source_dir: matches.value_of("IN_DIR")
                .map(|s| {
                    ({
                         let mut src = s.to_string();
                         if !['/', '\\'].contains(&src.chars().last().unwrap()) {
                             src.push('/');
                         }
                         src
                     },
                     PathBuf::from(s).canonicalize().unwrap())
                })
                .unwrap(),
            output_dir: matches.value_of("OUT_DIR")
                .map(|o| {
                    {
                        let mut p = PathBuf::from(&o);
                        if !p.is_absolute() {
                            p = PathBuf::from(format!("./{}", o));
                        }
                        if p.exists() {
                            if !matches.is_present("force") {
                                ClapError {
                                        message: format!("Output directory \"{}\" already exists", p.display()),
                                        kind: ClapErrorKind::InvalidValue,
                                        info: None,
                                    }
                                    .exit();
                            } else {
                                fs::remove_dir_all(p).expect("failed to remove preexisting output directory");
                            }
                        }
                    }

                    ({
                         let mut out = o.to_string();
                         if !['/', '\\'].contains(&out.chars().last().unwrap()) {
                             out.push('/');
                         }
                         out
                     },
                     {
                         let fname = Path::new(&o).file_name().unwrap();
                         let mut p = PathBuf::from(&o);
                         if !p.is_absolute() {
                             p = PathBuf::from(format!("./{}", o));
                         }
                         p.parent().unwrap().canonicalize().unwrap_or_else(|_| Path::new(".").canonicalize().unwrap()).join(fname)
                     })
                })
                .unwrap(),
        }
    }

    fn source_directory_validator(s: String) -> Result<(), String> {
        fs::canonicalize(&s).map_err(|_| format!("Input directory \"{}\" not found", s)).and_then(|f| if f.is_file() {
            Err(format!("Input directory \"{}\" not actualy a directory", s))
        } else {
            Ok(())
        })
    }

    fn output_directory_validator(s: String) -> Result<(), String> {
        let mut p = PathBuf::from(&s);
        if !p.is_absolute() {
            p = PathBuf::from(format!("./{}", s));
        }
        if p.parent().is_some() {
            p.pop();
            fs::canonicalize(&p).map_err(|_| format!("Output directory's parent directory \"{}\" nonexistant", p.display())).and_then(|f| if !f.is_file() {
                Ok(())
            } else {
                Err(format!("Output directory's parent directory \"{}\" actually a file", p.display()))
            })
        } else {
            Ok(())
        }
    }
}
