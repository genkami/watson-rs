use std::fs;
use std::io;
use std::path;
use std::rc::Rc;

use crate::error::*;
use crate::language::{Insn, Location, Mode, Token};

// No specific reason to use this value.
const BUF_SIZE: usize = 256;

/// A lexer of the WATSON language.
pub struct Lexer<R: io::Read> {
    reader: R,
    buf: Vec<u8>,
    filled: usize,
    current: usize,

    mode: Mode,

    last_read_ascii: u8,
    file_path: Option<Rc<path::Path>>,
    line: usize,
    column: usize,
}

/// A builder for `Lexer`.
pub struct Builder {
    initial_mode: Mode,
    file_path: Option<Rc<path::Path>>,
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
    pub fn file_path(mut self, path: &path::Path) -> Self {
        self.file_path = Some(path.to_path_buf().into());
        self
    }

    /// Builds a new `Lexer` that reads from `reader`.
    pub fn build<R: io::Read>(mut self, reader: R) -> Lexer<R> {
        let mode = self.initial_mode;
        let path = self.file_path.take();
        self.build_internal(reader, path, mode)
    }

    /// Opens a file and builds a `Lexer` that reads from the given file.
    pub fn open(mut self, path: path::PathBuf) -> Result<Lexer<fs::File>> {
        let file = fs::File::open(&path).into_watson_result(|| Location::unknown())?;
        let path_to_display = self
            .file_path
            .take()
            .or_else(|| Some(path.to_path_buf().into()));
        let mode = self.initial_mode;
        Ok(self.build_internal(file, path_to_display, mode))
    }

    fn build_internal<R: io::Read>(
        self,
        reader: R,
        path: Option<Rc<path::Path>>,
        mode: Mode,
    ) -> Lexer<R> {
        Lexer {
            reader: reader,
            buf: vec![0; BUF_SIZE],
            filled: 0,
            current: 0,
            mode: mode,
            last_read_ascii: 0,
            file_path: path,
            line: 1,
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
                        location: Location {
                            ascii: byte,
                            path: self.file_path.clone(),
                            line: self.line,
                            column: self.column,
                        },
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
        self.last_read_ascii = byte;
        if byte == b'\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
        Ok(byte)
    }

    fn fill_buffer(&mut self) -> Result<()> {
        self.filled = self
            .reader
            .read(&mut self.buf)
            .into_watson_result(|| self.current_location())?;
        Ok(())
    }

    fn current_location(&self) -> Location {
        Location {
            ascii: self.last_read_ascii,
            path: self.file_path.as_ref().map(|rc| Rc::clone(rc)),
            line: self.line,
            column: self.column,
        }
    }

    fn advance_state(&mut self, insn: Insn) {
        match insn {
            // See https://github.com/genkami/watson/blob/main/doc/spec.md#watson-representation.
            Insn::Snew => {
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

    #[test]
    fn builder_initial_mode_defaults_to_a() {
        let asciis = b"Bubba".to_vec();
        let mut lexer = Builder::new().build(&asciis[..]);
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Inew,
                location: Location {
                    ascii: b'B',
                    path: None,
                    line: 1,
                    column: 1,
                },
            },
        );
    }

    #[test]
    fn builder_initial_mode_can_be_overridden() {
        let asciis = b"Shaak".to_vec();
        let mut lexer = Builder::new().initial_mode(Mode::S).build(&asciis[..]);
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Inew,
                location: Location {
                    ascii: b'S',
                    path: None,
                    line: 1,
                    column: 1,
                },
            },
        );
    }

    #[test]
    fn builder_file_path_defaults_to_none() {
        let asciis = b"Bubba".to_vec();
        let mut lexer = Builder::new().build(&asciis[..]);
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Inew,
                location: Location {
                    ascii: b'B',
                    path: None,
                    line: 1,
                    column: 1,
                },
            },
        );
    }

    #[test]
    fn builder_file_path_can_be_overridden() {
        let asciis = b"Bubba".to_vec();
        let path = path::Path::new("test.watson");
        let mut lexer = Builder::new().file_path(path).build(&asciis[..]);
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Inew,
                location: Location {
                    ascii: b'B',
                    path: Some(path.to_path_buf().into()),
                    line: 1,
                    column: 1,
                },
            },
        );
    }

    #[test]
    fn builder_open_opens_a_file() -> Result<()> {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tempfile = NamedTempFile::new().into_watson_result(|| Location::unknown())?;
        tempfile
            .write_all(b"Bubba")
            .into_watson_result(|| Location::unknown())?;
        let path = tempfile.into_temp_path();

        let mut lexer = Builder::new().open(path.to_path_buf())?;
        assert_eq!(
            lexer.next_token()?,
            Token {
                insn: Insn::Inew,
                location: Location {
                    ascii: b'B',
                    path: Some(path.to_path_buf().into()),
                    line: 1,
                    column: 1,
                },
            },
        );
        Ok(())
    }

    #[test]
    fn builder_open_path_can_be_overridden() -> Result<()> {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tempfile = NamedTempFile::new().into_watson_result(|| Location::unknown())?;
        tempfile
            .write_all(b"Bubba")
            .into_watson_result(|| Location::unknown())?;
        let path = tempfile.into_temp_path();
        let path_to_display = path::Path::new("anothername.watson");

        let mut lexer = Builder::new()
            .file_path(path_to_display)
            .open(path.to_path_buf())?;
        assert_eq!(
            lexer.next_token()?,
            Token {
                insn: Insn::Inew,
                location: Location {
                    ascii: b'B',
                    path: Some(path_to_display.to_path_buf().into()),
                    line: 1,
                    column: 1,
                },
            },
        );
        Ok(())
    }

    #[test]
    fn lexer_advances_column_and_line() {
        let asciis = b"Bub\nba".to_vec();
        let mut lexer = Builder::new().build(&asciis[..]);
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Inew,
                location: Location {
                    ascii: b'B',
                    path: None,
                    line: 1,
                    column: 1,
                },
            },
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Iinc,
                location: Location {
                    ascii: b'u',
                    path: None,
                    line: 1,
                    column: 2,
                },
            },
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Ishl,
                location: Location {
                    ascii: b'b',
                    path: None,
                    line: 1,
                    column: 3,
                },
            },
        );

        // lexer hits \n here
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Ishl,
                location: Location {
                    ascii: b'b',
                    path: None,
                    line: 2,
                    column: 1,
                },
            },
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Iadd,
                location: Location {
                    ascii: b'a',
                    path: None,
                    line: 2,
                    column: 2,
                },
            },
        );
    }

    #[test]
    fn lexer_changes_mode() {
        let asciis = b"Bu?Sh$B".to_vec();
        let mut lexer = Builder::new().build(&asciis[..]);
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Inew,
                location: Location {
                    ascii: b'B',
                    path: None,
                    line: 1,
                    column: 1,
                },
            },
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Iinc,
                location: Location {
                    ascii: b'u',
                    path: None,
                    line: 1,
                    column: 2,
                },
            },
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Snew,
                location: Location {
                    ascii: b'?',
                    path: None,
                    line: 1,
                    column: 3,
                },
            },
        );

        // Lexer hits `Onew`, so it changes its mode to `S`.
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Inew,
                location: Location {
                    ascii: b'S',
                    path: None,
                    line: 1,
                    column: 4,
                },
            },
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Iinc,
                location: Location {
                    ascii: b'h',
                    path: None,
                    line: 1,
                    column: 5,
                },
            },
        );
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Snew,
                location: Location {
                    ascii: b'$',
                    path: None,
                    line: 1,
                    column: 6,
                },
            },
        );
        // Lexer hits `Onew`, so it changes its mode to `A`.
        assert_eq!(
            lexer.next_token().unwrap(),
            Token {
                insn: Insn::Inew,
                location: Location {
                    ascii: b'B',
                    path: None,
                    line: 1,
                    column: 7,
                },
            },
        );
    }
}
