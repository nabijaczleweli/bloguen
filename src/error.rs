use self::super::util::uppercase_first;
use std::error::Error as StdError;
use std::borrow::Cow;
use std::fmt;


/// Enum representing all possible ways the application can fail.
///
/// # Examples
///
/// ```
/// # use bloguen::Error;
/// assert_eq!(Error::Io {
///                desc: "network".into(),
///                op: "write",
///                more: "full buffer".into(),
///            }.to_string(),
///            "Writing network failed: full buffer.");
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Error {
    /// An I/O error occured.
    ///
    /// This includes higher-level I/O errors like FS ones.
    Io {
        /// The file the I/O operation regards.
        desc: Cow<'static, str>,
        /// The failed operation.
        ///
        /// This should be lowercase and imperative ("create", "open").
        op: &'static str,
        /// Additional data.
        more: Cow<'static, str>,
    },
    Parse {
        /// What failed to parse.
        ///
        /// Something like "URL", "datetime".
        tp: &'static str,
        /// Where the thing that failed to parse would go, were it to parse properly.
        wher: Cow<'static, str>,
        /// Additional data.
        more: Cow<'static, str>,
    },
    /// A requested file doesn't exist.
    FileNotFound {
        /// What requested the file.
        who: &'static str,
        /// The file that should exist.
        path: Cow<'static, str>,
    },
    /// A path is in a wrong state.
    WrongFileState {
        /// What the file is not.
        what: &'static str,
        /// The file that should be.
        path: Cow<'static, str>,
    },
    /// Failed to parse the specified file because of the specified error(s).
    FileParsingFailed {
        /// The file that failed to parse.
        desc: Cow<'static, str>,
        /// The parsing error(s) that occured.
        errors: Cow<'static, str>,
    },
}

impl Error {
    /// Get the executable exit value from an `Error` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bloguen::Error;
    /// assert_eq!(Error::Io {
    ///     desc: "".into(),
    ///     op: "",
    ///     more: "".into(),
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

impl fmt::Display for Error {
    fn fmt(&self, err_out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Io { ref desc, op, ref more } => {
                // Strip the last 'e', if any, so we get correct inflection for continuous tenses
                let op = uppercase_first(if op.ends_with('e') {
                    &op[..op.len() - 1]
                } else {
                    op
                });

                write!(err_out, "{}ing {} failed: {}.", op, desc, more)
            }
            Error::Parse { tp, ref wher, ref more } => write!(err_out, "Failed to parse {} for {}: {}.", tp, wher, more),
            Error::FileNotFound { who, ref path } => write!(err_out, "File {} for {} not found.", path, who),
            Error::WrongFileState { what, ref path } => write!(err_out, "File {} is not {}.", path, what),
            Error::FileParsingFailed { ref desc, ref errors } => write!(err_out, "Failed to parse {}: {}.", desc, errors),
        }
    }
}

impl StdError for Error {}
