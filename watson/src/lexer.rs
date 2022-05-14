use std::fs;
use std::io;
use std::path;

use crate::vm;

// No specific reason to use this value.
const BUF_SIZE: usize = 256;

mod conv {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    use crate::vm;

    // See https://github.com/genkami/watson/blob/main/doc/spec.md#watson-representation.
    pub static ASCII_TO_INSN_TABLE_A: Lazy<HashMap<u8, vm::Insn>> =
        Lazy::new(|| build_ascii_to_insn_map(b"BubaAei'qtp?!~M@szo.E#%"));

    pub static ASCII_TO_INSN_TABLE_S: Lazy<HashMap<u8, vm::Insn>> =
        Lazy::new(|| build_ascii_to_insn_map(b"ShakrAzimbu$-+gv?^!y/e:"));

    pub static INSN_TO_ASCII_TABLE_A: Lazy<HashMap<vm::Insn, u8>> =
        Lazy::new(|| reverse(&*ASCII_TO_INSN_TABLE_A));

    pub static INSN_TO_ASCII_TABLE_S: Lazy<HashMap<vm::Insn, u8>> =
        Lazy::new(|| reverse(&*ASCII_TO_INSN_TABLE_S));

    fn build_ascii_to_insn_map(asciis: &[u8]) -> HashMap<u8, vm::Insn> {
        asciis
            .iter()
            .zip(&vm::ALL_INSNS)
            .map(|(c, i)| (*c, *i))
            .collect()
    }

    fn reverse(orig: &HashMap<u8, vm::Insn>) -> HashMap<vm::Insn, u8> {
        orig.iter().map(|(c, i)| (*i, *c)).collect()
    }
}

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
    pub fn flip(self) -> Mode {
        match self {
            Mode::A => Mode::S,
            Mode::S => Mode::A,
        }
    }

    // Converts an ASCII character to its corresponding `vm::Insn` with respect to the current `Mode`.
    pub fn ascii_to_insn(self, ascii: u8) -> Option<vm::Insn> {
        let table = match self {
            Mode::A => &*conv::ASCII_TO_INSN_TABLE_A,
            Mode::S => &*conv::ASCII_TO_INSN_TABLE_S,
        };
        table.get(&ascii).map(|i| *i)
    }

    // Converts a `vm::Insn` to its corresponding ASCII character with respect to the current `Mode`.
    pub fn insn_to_ascii(self, insn: vm::Insn) -> u8 {
        let table = match self {
            Mode::A => &*conv::INSN_TO_ASCII_TABLE_A,
            Mode::S => &*conv::INSN_TO_ASCII_TABLE_S,
        };
        table.get(&insn).map(|c| *c).unwrap()
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
            match self.mode.ascii_to_insn(byte) {
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
        let byte = self.buf[cur];
        if byte == b'\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
        Ok(byte)
    }

    fn fill_buffer(&mut self) -> Result<()> {
        self.filled = self.reader.read(&mut self.buf)?;
        Ok(())
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

#[cfg(test)]
mod test {
    use super::*;

    // 0x21 to 0x7E
    const ASCII_CHARS: std::ops::RangeInclusive<u8> = b'!'..=b'~';

    #[test]
    fn mode_ascii_to_insn_is_surjective() {
        fn assert_surjective(mode: Mode) {
            use std::collections::HashSet;

            let mut insns = vm::ALL_INSNS.iter().map(|i| *i).collect::<HashSet<_>>();
            for c in ASCII_CHARS {
                mode.ascii_to_insn(c).map(|insn| insns.remove(&insn));
            }
            for insn in insns {
                panic!(
                    "mode={:?}: instruction {:?} does not have matching ASCII characters",
                    mode, insn
                );
            }
        }

        assert_surjective(Mode::A);
        assert_surjective(Mode::S);
    }

    #[test]
    fn mode_ascii_to_insn_is_injective() {
        fn assert_injective(mode: Mode) {
            use std::collections::HashMap;

            let mut reversed = HashMap::new();
            for c in ASCII_CHARS {
                mode.ascii_to_insn(c).map(|insn| match reversed.get(&insn) {
                    None => {
                        reversed.insert(insn, c);
                    }
                    Some(d) => {
                        panic!(
                            "mode={:?}: both {:?} and {:?} are converted into {:?}",
                            mode, c, d, insn
                        );
                    }
                });
            }
        }

        assert_injective(Mode::A);
        assert_injective(Mode::S);
    }

    #[test]
    fn mode_insn_to_ascii_never_panics() {
        fn assert_never_panics(mode: Mode) {
            for i in vm::ALL_INSNS {
                mode.insn_to_ascii(i);
            }
        }

        assert_never_panics(Mode::A);
        assert_never_panics(Mode::S);
    }

    #[test]
    fn mode_insn_to_ascii_is_injective() {
        fn assert_injective(mode: Mode) {
            use std::collections::HashMap;

            let mut reversed = HashMap::new();
            for i in vm::ALL_INSNS {
                let c = mode.insn_to_ascii(i);
                match reversed.get(&c) {
                    None => {
                        reversed.insert(c, i);
                    }
                    Some(j) => {
                        panic!(
                            "mode={:?}: both {:?} and {:?} are converted into {:?}",
                            mode, i, j, c
                        );
                    }
                }
            }
        }

        assert_injective(Mode::A);
        assert_injective(Mode::S);
    }
}
