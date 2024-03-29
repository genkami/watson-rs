use std::error::Error as StdError;
use std::fmt;

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

/// Error represents an error when serializing to or deserializing from WATSON.
#[derive(Debug)]
pub struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) location: Option<watson_rs::Location>,
    pub(crate) source: Option<Box<dyn StdError>>,
}

impl Error {
    /// Returns its `ErrorKind`.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Returns an optional `Location` where the error happened.
    pub fn location(&self) -> Option<&watson_rs::Location> {
        self.location.as_ref()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.location() {
            Some(loc) => write!(f, "{} at {}", self.kind, loc),
            None => write!(f, "{} at unknown location", self.kind),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_deref()
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Error {
            kind: ErrorKind::Custom(format!("{msg}")),
            location: None,
            source: None,
        }
    }
}
impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        ser::Error::custom(msg)
    }
}

impl From<watson_rs::Error> for Error {
    fn from(err: watson_rs::Error) -> Self {
        Error {
            kind: ErrorKind::ExecutionError(err.kind),
            location: Some(err.location.clone()),
            source: Some(Box::new(err)),
        }
    }
}

impl Error {
    pub(crate) fn key_must_be_bytes() -> Self {
        Error {
            kind: ErrorKind::KeyMustBeBytes,
            location: None,
            source: None,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ErrorKind {
    /// Object key can't be converted into bytes.
    KeyMustBeBytes,

    /// Unexpected map key detected while deserializing.
    UnexpectedMapKey,

    /// Unexpected map value detected while deserializing.
    UnexpectedMapValue,

    /// Unexpected map detected while deserializing.
    UnexpectedMap,

    /// An error occurred during VM execution.
    ExecutionError(watson_rs::error::ErrorKind),

    /// A user-defined error.
    Custom(String),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ErrorKind::KeyMustBeBytes => write!(f, "Key must be bytes"),
            ErrorKind::UnexpectedMapKey => write!(f, "Unexpected map key"),
            ErrorKind::UnexpectedMapValue => write!(f, "Unexpected map value"),
            ErrorKind::UnexpectedMap => write!(f, "Unexpected map"),
            ErrorKind::ExecutionError(ref k) => k.fmt(f),
            ErrorKind::Custom(ref s) => write!(f, "{s}"),
        }
    }
}
