use std::collections::HashMap;
use std::fs;
use std::io;
use std::path;

use once_cell::sync::Lazy;

use crate::vm;
use crate::vm::Insn::*;

// No specific reason to use this value.
const BUF_SIZE: usize = 256;

static ALL_INSNS: [vm::Insn; 23] = [
    Inew, Iinc, Ishl, Iadd, Ineg, Isht, Itof, Itou, Finf, Fnan, Fneg, Snew, Sadd, Onew, Oadd, Anew,
    Aadd, Bnew, Bneg, Nnew, Gdup, Gpop, Gswp,
];

// See https://github.com/genkami/watson/blob/main/doc/spec.md#watson-representation.
static INSN_MAP_A: Lazy<HashMap<u8, vm::Insn>> = Lazy::new(|| {
    let a_chars = b"BubaAei'qtp?!~M@sqo.E#%";
    let mut m = HashMap::new();
    for (c, i) in a_chars.iter().zip(&ALL_INSNS) {
        m.insert(*c, *i);
    }
    m
});
static INSN_MAP_S: Lazy<HashMap<u8, vm::Insn>> = Lazy::new(|| {
    let s_chars = b"ShakrAzimbu$-+gv?^!y/e:";
    let mut m = HashMap::new();
    for (c, i) in s_chars.iter().zip(&ALL_INSNS) {
        m.insert(*c, *i);
    }
    m
});

/// A "mode" of the WATSON lexer.
/// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
#[derive(Copy, Clone, Debug)]
pub enum Mode {
    /// The A mode.
    A,
    /// The S mode.
    S,
}

impl Mode {
    /// Returns the opposite state.
    fn flip(self) -> Mode {
        match self {
            Mode::A => Mode::S,
            Mode::S => Mode::A,
        }
    }
}

/// A token of the WATSON language.
pub struct Token {
    /// A VM instruction that the token represents.
    pub insn: vm::Insn,

    /// Location of the instrution.
    pub file_path: Option<path::PathBuf>,
    pub line: usize,
    pub column: usize,
}

/// A lexer of the WATSON language.
pub struct Lexer<R: io::Read> {
    reader: R,
    buf: Vec<u8>,
    filled: usize,
    current: usize,

    mode: Mode,

    // TODO: this should be Option<Rc<Path>> or something
    file_path: Option<path::PathBuf>,
    line: usize,
    column: usize,
}

/// The error type for `Lexer`.
pub enum Error {
    IOError(io::Error),
    Unexpected,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IOError(e)
    }
}

/// The specialized `Result` type for `Lexer`.
pub type Result<T> = std::result::Result<T, Error>;

/// A builder for `Lexer`.
pub struct Builder {
    initial_mode: Mode,
    file_path: Option<path::PathBuf>,
}

impl Builder {
    /// Returns a new `Builder`.
    pub fn new() -> Self {
        Builder {
            initial_mode: Mode::A,
            file_path: None,
        }
    }

    /// Sets an initial `Mode`.
    pub fn initial_mode(mut self, mode: Mode) -> Self {
        self.initial_mode = mode;
        self
    }

    /// Sets a file path to display.
    pub fn file_path(mut self, path: path::PathBuf) -> Self {
        self.file_path = Some(path);
        self
    }

    /// Builds a new `Lexer` that reads from `reader`.
    pub fn build<R: io::Read>(mut self, reader: R) -> Lexer<R> {
        let mode = self.initial_mode;
        let path = self.file_path.take();
        self.build_internal(reader, path, mode)
    }

    /// Opens a file and builds a `Lexer` that reads from the given file.
    pub fn open(mut self, path: path::PathBuf) -> io::Result<Lexer<fs::File>> {
        let file = fs::File::open(&path)?;
        let path_to_display = match self.file_path.take() {
            None => Some(path),
            Some(p) => Some(p),
        };
        let mode = self.initial_mode;
        Ok(self.build_internal(file, path_to_display, mode))
    }

    fn build_internal<R: io::Read>(
        self,
        reader: R,
        path: Option<path::PathBuf>,
        mode: Mode,
    ) -> Lexer<R> {
        Lexer {
            reader: reader,
            buf: vec![0; BUF_SIZE],
            filled: 0,
            current: 0,
            mode: mode,
            file_path: path,
            line: 0,
            column: 0,
        }
    }
}

impl<R: io::Read> Lexer<R> {
    /// Returns a next token if exists.
    pub fn next_token(&mut self) -> Result<Token> {
        let token: Token;
        loop {
            let byte = self.next_byte()?;
            match self.byte_to_insn(byte) {
                None => {
                    continue;
                }
                Some(insn) => {
                    token = Token {
                        insn: insn,
                        file_path: self.file_path.clone(),
                        line: self.line,
                        column: self.column,
                    };
                    break;
                }
            }
        }
        self.advance_state(token.insn);
        Ok(token)
    }

    fn next_byte(&mut self) -> Result<u8> {
        if self.filled <= self.current {
            self.fill_buffer()?;
        }
        let cur = self.current;
        self.current += 1;
        Ok(self.buf[cur])
    }

    fn fill_buffer(&mut self) -> Result<()> {
        self.filled = self.reader.read(&mut self.buf)?;
        Ok(())
    }

    fn byte_to_insn(&self, byte: u8) -> Option<vm::Insn> {
        let m = match self.mode {
            Mode::A => &*INSN_MAP_A,
            Mode::S => &*INSN_MAP_S,
        };
        m.get(&byte).map(|insn| *insn)
    }

    fn advance_state(&mut self, insn: vm::Insn) {
        match insn {
            // See https://github.com/genkami/watson/blob/main/doc/spec.md#watson-representation.
            vm::Insn::Snew => {
                self.mode = self.mode.flip();
            }
            _ => {
                // nop
            }
        }
    }
}
