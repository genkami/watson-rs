use std::fs;
use std::io;
use std::path;
use std::rc::Rc;

use crate::error::{Error, Result};
use crate::language::{Insn, Location, Mode, Token};
use crate::vm::ReadToken;

/// A lexer of the WATSON language.
pub struct Lexer<R> {
    bytes: io::Bytes<R>,

    mode: Mode,

    last_read_byte: u8,
    file_path: Option<Rc<path::Path>>,
    line: usize,
    column: usize,
}

/// Config configures a `Lexer`.
pub struct Config {
    // Initial mode of a `Lexer` (defaults to `A` by the specificaton).
    pub initial_mode: Mode,

    // File path to display (not used to open a file or something).
    pub file_path: Option<Rc<path::Path>>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            initial_mode: Mode::A,
            file_path: None,
        }
    }
}

impl Config {
    /// Returns a new `Lexer` that reads from the given reader.
    pub fn new<R: io::Read>(self, reader: R) -> Lexer<R> {
        Lexer {
            bytes: reader.bytes(),
            mode: self.initial_mode,
            last_read_byte: 0,
            file_path: self.file_path,
            line: 1,
            column: 0,
        }
    }

    /// Opens a file and builds a `Lexer` that reads from the given file.
    pub fn open(mut self, path: &path::Path) -> Result<Lexer<fs::File>> {
        let file = fs::File::open(&path)?;
        if self.file_path.is_none() {
            self.file_path = Some(path.to_path_buf().into());
        }
        Ok(self.new(file))
    }
}

impl Lexer<fs::File> {
    /// Opens a file and builds a `Lexer` with the default configuration.
    pub fn open(path: &path::Path) -> Result<Self> {
        Config::default().open(path)
    }
}

impl<R: io::Read> Lexer<R> {
    /// Returns a new `Lexer` with the default configuration.
    pub fn new(reader: R) -> Self {
        Config::default().new(reader)
    }

    /// Returns the next byte.
    /// EOF is mapped to `Ok(None)`.
    fn next_byte(&mut self) -> Result<Option<u8>> {
        match self.bytes.next() {
            None => Ok(None),
            Some(byte) => {
                let byte = byte.map_err(|e| Error::from_io_error(e, self.current_location()))?;
                self.last_read_byte = byte;
                if byte == b'\n' {
                    self.line += 1;
                    self.column = 0;
                } else {
                    self.column += 1;
                }
                Ok(Some(byte))
            }
        }
    }

    fn current_location(&self) -> Location {
        Location {
            byte: self.last_read_byte,
            path: self.file_path.as_ref().map(Rc::clone),
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

impl<R: io::Read> ReadToken for Lexer<R> {
    /// Returns a next token if exists.
    fn read(&mut self) -> Result<Option<Token>> {
        let token: Token;
        loop {
            let byte = self.next_byte()?;
            match byte {
                None => {
                    return Ok(None);
                }
                Some(byte) => match Insn::from_byte(self.mode, byte) {
                    None => {
                        continue;
                    }
                    Some(insn) => {
                        token = Token {
                            insn,
                            location: Location {
                                byte,
                                path: self.file_path.clone(),
                                line: self.line,
                                column: self.column,
                            },
                        };
                        self.advance_state(token.insn);
                        return Ok(Some(token));
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lexer_new_initial_mode_defaults_to_a() {
        let bytes = b"Bubba".to_vec();
        let mut lexer = Lexer::new(&bytes[..]);
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Inew,
                location: Location {
                    byte: b'B',
                    path: None,
                    line: 1,
                    column: 1,
                },
            }),
        );
    }

    #[test]
    fn lexer_new_initial_mode_can_be_overridden() {
        let bytes = b"Shaak".to_vec();
        let mut conf = Config::default();
        conf.initial_mode = Mode::S;
        let mut lexer = conf.new(&bytes[..]);
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Inew,
                location: Location {
                    byte: b'S',
                    path: None,
                    line: 1,
                    column: 1,
                },
            }),
        );
    }

    #[test]
    fn lexer_new_file_path_defaults_to_none() {
        let bytes = b"Bubba".to_vec();
        let mut lexer = Lexer::new(&bytes[..]);
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Inew,
                location: Location {
                    byte: b'B',
                    path: None,
                    line: 1,
                    column: 1,
                },
            }),
        );
    }

    #[test]
    fn lexer_file_path_can_be_overridden() {
        let bytes = b"Bubba".to_vec();
        let path = path::Path::new("test.watson");
        let mut conf = Config::default();
        conf.file_path = Some(path.to_path_buf().into());
        let mut lexer = conf.new(&bytes[..]);
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Inew,
                location: Location {
                    byte: b'B',
                    path: Some(path.to_path_buf().into()),
                    line: 1,
                    column: 1,
                },
            }),
        );
    }

    #[test]
    fn lexer_open_opens_a_file() -> Result<()> {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tempfile = NamedTempFile::new()?;
        tempfile.write_all(b"Bubba")?;
        let path = tempfile.into_temp_path();

        let mut lexer = Lexer::open(&path)?;
        assert_eq!(
            lexer.read()?,
            Some(Token {
                insn: Insn::Inew,
                location: Location {
                    byte: b'B',
                    path: Some(path.to_path_buf().into()),
                    line: 1,
                    column: 1,
                },
            }),
        );
        Ok(())
    }

    #[test]
    fn lexer_open_path_can_be_overridden() -> Result<()> {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tempfile = NamedTempFile::new()?;
        tempfile.write_all(b"Bubba")?;
        let path = tempfile.into_temp_path();
        let path_to_display = path::Path::new("anothername.watson");

        let mut conf = Config::default();
        conf.file_path = Some(path_to_display.to_path_buf().into());
        let mut lexer = conf.open(&path)?;
        assert_eq!(
            lexer.read()?,
            Some(Token {
                insn: Insn::Inew,
                location: Location {
                    byte: b'B',
                    path: Some(path_to_display.to_path_buf().into()),
                    line: 1,
                    column: 1,
                },
            }),
        );
        Ok(())
    }

    #[test]
    fn lexer_advances_column_and_line() {
        let bytes = b"Bub\nba".to_vec();
        let mut lexer = Lexer::new(&bytes[..]);
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Inew,
                location: Location {
                    byte: b'B',
                    path: None,
                    line: 1,
                    column: 1,
                },
            }),
        );
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Iinc,
                location: Location {
                    byte: b'u',
                    path: None,
                    line: 1,
                    column: 2,
                },
            }),
        );
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Ishl,
                location: Location {
                    byte: b'b',
                    path: None,
                    line: 1,
                    column: 3,
                },
            }),
        );

        // lexer hits \n here
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Ishl,
                location: Location {
                    byte: b'b',
                    path: None,
                    line: 2,
                    column: 1,
                },
            }),
        );
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Iadd,
                location: Location {
                    byte: b'a',
                    path: None,
                    line: 2,
                    column: 2,
                },
            }),
        );
    }

    #[test]
    fn lexer_returns_none_when_eof() {
        let bytes = b"Bub".to_vec();
        let mut lexer = Lexer::new(&bytes[..]);

        assert_eq!(lexer.read().unwrap().unwrap().insn, Insn::Inew);
        assert_eq!(lexer.read().unwrap().unwrap().insn, Insn::Iinc);
        assert_eq!(lexer.read().unwrap().unwrap().insn, Insn::Ishl);
        assert_eq!(lexer.read().unwrap(), None);
    }

    #[test]
    fn lexer_changes_mode() {
        let bytes = b"Bu?Sh$B".to_vec();
        let mut lexer = Lexer::new(&bytes[..]);
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Inew,
                location: Location {
                    byte: b'B',
                    path: None,
                    line: 1,
                    column: 1,
                },
            }),
        );
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Iinc,
                location: Location {
                    byte: b'u',
                    path: None,
                    line: 1,
                    column: 2,
                },
            }),
        );
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Snew,
                location: Location {
                    byte: b'?',
                    path: None,
                    line: 1,
                    column: 3,
                },
            }),
        );

        // Lexer hits `Onew`, so it changes its mode to `S`.
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Inew,
                location: Location {
                    byte: b'S',
                    path: None,
                    line: 1,
                    column: 4,
                },
            }),
        );
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Iinc,
                location: Location {
                    byte: b'h',
                    path: None,
                    line: 1,
                    column: 5,
                },
            }),
        );
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Snew,
                location: Location {
                    byte: b'$',
                    path: None,
                    line: 1,
                    column: 6,
                },
            }),
        );
        // Lexer hits `Onew`, so it changes its mode to `A`.
        assert_eq!(
            lexer.read().unwrap(),
            Some(Token {
                insn: Insn::Inew,
                location: Location {
                    byte: b'B',
                    path: None,
                    line: 1,
                    column: 7,
                },
            }),
        );
    }
}
