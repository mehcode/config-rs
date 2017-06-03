use std::error::Error;
use std::borrow::Cow;
use std::result;
use std::fmt;
use serde::de;
use nom;

#[derive(Debug)]
pub enum Unexpected {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Str(String),
    Unit,
    Seq,
    Map
}

impl fmt::Display for Unexpected {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            Unexpected::Bool(b) => write!(f, "boolean `{}`", b),
            Unexpected::Integer(i) => write!(f, "integer `{}`", i),
            Unexpected::Float(v) => write!(f, "floating point `{}`", v),
            Unexpected::Str(ref s) => write!(f, "string {:?}", s),
            Unexpected::Unit => write!(f, "unit value"),
            Unexpected::Seq => write!(f, "sequence"),
            Unexpected::Map => write!(f, "map"),
        }
    }
}

/// Represents all possible errors that can occur when working with
/// configuration.
pub enum ConfigError {
    /// Configuration is frozen and no further mutations can be made.
    Frozen,

    /// Configuration property was not found
    NotFound(String),

    /// Configuration path could not be parsed.
    PathParse(nom::ErrorKind),

    /// Configuration could not be parsed from file.
    FileParse { uri: Option<String>, cause: Box<Error> },

    /// Value could not be converted into the requested type.
    Type {
        origin: Option<String>,
        unexpected: Unexpected,
        expected: &'static str,
        key: Option<String>,
    },

    /// Custom message
    Message(String),

    /// Unadorned error from a foreign origin.
    Foreign(Box<Error>),
}

impl ConfigError {
    // FIXME: pub(crate)
    #[doc(hidden)]
    pub fn invalid_type(origin: Option<String>, unexpected: Unexpected, expected: &'static str) -> Self {
        ConfigError::Type {
            origin: origin,
            unexpected: unexpected,
            expected: expected,
            key: None,
         }
    }

    // FIXME: pub(crate)
    #[doc(hidden)]
    pub fn extend_with_key(self, key: &str) -> Self {
        match self {
            ConfigError::Type { origin, unexpected, expected, .. } => {
                ConfigError::Type {
                    origin: origin,
                    unexpected: unexpected,
                    expected: expected,
                    key: Some(key.into()),
                }
            }

            _ => self,
        }
    }
}

/// Alias for a `Result` with the error type set to `ConfigError`.
pub type Result<T> = result::Result<T, ConfigError>;

// Forward Debug to Display for readable panic! messages
impl fmt::Debug for ConfigError {
    fn fmt(&self, f:  &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self)
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::Frozen | ConfigError::PathParse(_) => {
                write!(f, "{}", self.description())
            }

            ConfigError::Message(ref s) => {
                write!(f, "{}", s)
            }

            ConfigError::Foreign(ref cause) => {
                write!(f, "{}", cause)
            }

            ConfigError::NotFound(ref key) => {
                write!(f, "configuration property {:?} not found", key)
            }

            ConfigError::Type { ref origin, ref unexpected, expected, ref key } => {
                write!(f, "invalid type: {}, expected {}",
                    unexpected, expected)?;

                if let Some(ref key) = *key {
                    write!(f, " for key `{}`", key)?;
                }

                if let Some(ref origin) = *origin {
                    write!(f, " in {}", origin)?;
                }

                Ok(())
            }

            ConfigError::FileParse { ref cause, ref uri } => {
                write!(f, "{}", cause)?;

                if let Some(ref uri) = *uri {
                    write!(f, " in {}", uri)?;
                }

                Ok(())
            }
        }
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            ConfigError::Frozen => "configuration is frozen",
            ConfigError::NotFound(_) => "configuration property not found",
            ConfigError::Type { .. } => "invalid type",
            ConfigError::Foreign(ref cause) => cause.description(),
            ConfigError::FileParse { ref cause, .. } => cause.description(),
            ConfigError::PathParse(ref kind) => kind.description(),

            _ => "configuration error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ConfigError::Foreign(ref cause) => Some(cause.as_ref()),
            ConfigError::FileParse { ref cause, .. } => Some(cause.as_ref()),

            _ => None
        }
    }
}

impl de::Error for ConfigError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        ConfigError::Message(msg.to_string())
    }
}
