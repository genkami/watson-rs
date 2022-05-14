use std::fmt;
use std::path;
use std::rc::Rc;

/// The error type of the WATSON VM.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Error {
    /// Represents details of the error.
    pub kind: ErrorKind,

    /// The location where the error happened.
    pub location: Location,
}

/// Location where an error happened.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Location {
    /// A character that the WATSON VM read.
    pub ascii: u8,

    /// Optional file path.
    pub path: Option<Rc<path::Path>>,

    /// Line number.
    pub line: usize,

    /// Column number.
    pub column: usize,
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
