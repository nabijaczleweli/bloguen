use self::super::util::uppercase_first;
use std::path::PathBuf;
use std::io::Write;


/// Enum representing all possible ways the application can fail.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Error {
    /// An I/O error occured.
    ///
    /// This includes higher-level I/O errors like FS ones.
    Io {
        /// The file the I/O operation regards.
        desc: &'static str,
        /// The failed operation.
        ///
        /// This should be lowercase and imperative ("create", "open").
        op: &'static str,
        /// Additional data.
        more: Option<String>,
    },
    Parse {
        /// What failed to parse.
        ///
        /// Something like "URL", "datetime".
        tp: &'static str,
        /// Where the thing that failed to parse would go, were it to parse properly.
        wher: &'static str,
        /// Additional data.
        more: Option<&'static str>,
    },
    /// A requested file doesn't exist.
    FileNotFound {
        /// What requested the file.
        who: &'static str,
        /// The file that should exist.
        path: PathBuf,
    },
    /// A path is in a wrong state.
    WrongFileState {
        /// What the file is not.
        what: &'static str,
        /// The file that should be.
        path: PathBuf,
    },
    /// Failed to parse the specified file because of the specified error(s).
    FileParsingFailed {
        /// The file that failed to parse.
        desc: &'static str,
        /// The parsing error(s) that occured.
        errors: Option<String>,
    },
}

impl Error {
    /// Write the error message to the specified output stream.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::Error;
    /// # use std::iter::FromIterator;
    /// let mut out = Vec::new();
    /// Error::Io {
    ///     desc: "network",
    ///     op: "write",
    ///     more: Some("full buffer".to_string()),
    /// }.print_error(&mut out);
    /// assert_eq!(out, "Writing network failed: full buffer.\n".as_bytes());
    /// ```
    pub fn print_error<W: Write>(&self, err_out: &mut W) {
        match *self {
            Error::Io { desc, op, ref more } => {
                // Strip the last 'e', if any, so we get correct inflection for continuous tenses
                let op = uppercase_first(if op.ends_with('e') {
                    &op[..op.len() - 1]
                } else {
                    op
                });
                write!(err_out, "{}ing {} failed", op, desc).unwrap();
                if let Some(more) = more {
                    write!(err_out, ": {}", more).unwrap();
                }
                writeln!(err_out, ".").unwrap();
            }
            Error::Parse { tp, wher, more } => {
                write!(err_out, "Failed to parse {} for {}", tp, wher).unwrap();
                if let Some(more) = more {
                    write!(err_out, ": {}", more).unwrap();
                }
                writeln!(err_out, ".").unwrap();
            }
            Error::FileNotFound { who, ref path } => writeln!(err_out, "File {} for {} not found.", path.display(), who).unwrap(),
            Error::WrongFileState { what, ref path } => writeln!(err_out, "File {} is not {}.", path.display(), what).unwrap(),
            Error::FileParsingFailed { ref desc, ref errors } => {
                write!(err_out, "Failed to parse {}", desc).unwrap();
                if let Some(ref error) = errors {
                    write!(err_out, ": {}", error).unwrap();
                }
                writeln!(err_out, ".").unwrap();
            }
        }
    }

    /// Get the executable exit value from an `Error` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::Error;
    /// assert_eq!(Error::Io {
    ///     desc: "",
    ///     op: "",
    ///     more: None,
    /// }.exit_value(), 1);
    /// ```
    pub fn exit_value(&self) -> i32 {
        match *self {
            Error::Io { .. } => 1,
            Error::Parse { .. } => 2,
            Error::FileNotFound { .. } => 3,
            Error::WrongFileState { .. } => 4,
            Error::FileParsingFailed { .. } => 5,
        }
    }
}
