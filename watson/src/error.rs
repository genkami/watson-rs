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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;
        match self.location.path.as_ref() {
            Some(p) => {
                write!(f, " at {}", p.to_string_lossy())?;
            }
            None => {
                write!(f, " at unknown location")?;
            }
        }
        write!(
            f,
            " (line: {}, column: {})",
            self.location.line, self.location.column
        )?;
        if let Some(c) = char::from_u32(self.location.ascii as u32) {
            write!(f, ", near the character {}", c)?;
        }
        Ok(())
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

pub trait IntoWATSONResult {
    type Ok;
    fn into_watson_result<F: FnOnce() -> Location>(self, f: F) -> Result<Self::Ok>;
}

impl<T> IntoWATSONResult for io::Result<T> {
    type Ok = T;

    fn into_watson_result<F: FnOnce() -> Location>(self, f: F) -> Result<T> {
        self.map_err(|ioerr| Error {
            kind: ErrorKind::IOError,
            location: f(),
            source: Some(Box::new(ioerr)),
        })
    }
}
