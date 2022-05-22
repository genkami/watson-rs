use std::error;
use std::fmt;
use std::io;

use crate::language::Location;

/// The error type of the WATSON VM.
#[derive(Debug)]
pub struct Error {
    /// Represents details of the error.
    pub kind: ErrorKind,

    /// The location where the error happened.
    pub location: Location,

    /// The internal error that causes this error.
    pub source: Option<Box<dyn error::Error>>,
}

/// Details of the `Error`.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum ErrorKind {
    /// The VM tried to pop values from an empty stack.
    EmptyStack,

    /// The type of the value on the top of stack is different from the one that the instruction wants.
    TypeMismatch,

    /// An I/O error happened.
    IOError,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Creates a new `Error` caused by the given `io::Error`.
    pub fn from_io_error(e: io::Error, location: Location) -> Self {
        Error {
            kind: ErrorKind::IOError,
            location: location,
            source: Some(Box::new(e)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}", self.kind, self.location)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ErrorKind::EmptyStack => "Empty stack",
            ErrorKind::TypeMismatch => "Type mismatch",
            ErrorKind::IOError => "I/O error",
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_deref()
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::from_io_error(e, Location::unknown())
    }
}
