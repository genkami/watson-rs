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

impl Lexer<fs::File> {
    /// Opens a file and builds a `Lexer` that reads from the given file.
    pub fn open(path: &path::Path) -> Result<Self> {
        Lexer::open_with_config(path, Config::default())
    }

    /// Same as `open` but in a configurable way.
    pub fn open_with_config(path: &path::Path, mut conf: Config) -> Result<Self> {
        let file = fs::File::open(&path).into_watson_result(|| Location::unknown())?;
        if conf.file_path.is_none() {
            conf.file_path = Some(path.to_path_buf().into());
        }
        Ok(Lexer::new_with_config(file, conf))
    }
}

impl<R: io::Read> Lexer<R> {
    /// Returns a new `Lexer` that reads from the given reader.
    pub fn new(reader: R) -> Self {
        Lexer::new_with_config(reader, Config::default())
    }

    /// Same as `new` but in a configurable way.
    pub fn new_with_config(reader: R, conf: Config) -> Self {
        Lexer {
            reader: reader,
            buf: vec![0; BUF_SIZE],
            filled: 0,
            current: 0,
            mode: conf.initial_mode,
            last_read_ascii: 0,
            file_path: conf.file_path,
            line: 1,
            column: 0,
        }
    }

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
    fn lexer_new_initial_mode_defaults_to_a() {
        let asciis = b"Bubba".to_vec();
        let mut lexer = Lexer::new(&asciis[..]);
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
    fn lexer_new_initial_mode_can_be_overridden() {
        let asciis = b"Shaak".to_vec();
        let mut conf = Config::default();
        conf.initial_mode = Mode::S;
        let mut lexer = Lexer::new_with_config(&asciis[..], conf);
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
    fn lexer_new_file_path_defaults_to_none() {
        let asciis = b"Bubba".to_vec();
        let mut lexer = Lexer::new(&asciis[..]);
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
    fn lexer_file_path_can_be_overridden() {
        let asciis = b"Bubba".to_vec();
        let path = path::Path::new("test.watson");
        let mut conf = Config::default();
        conf.file_path = Some(path.to_path_buf().into());
        let mut lexer = Lexer::new_with_config(&asciis[..], conf);
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
    fn lexer_open_opens_a_file() -> Result<()> {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tempfile = NamedTempFile::new().into_watson_result(|| Location::unknown())?;
        tempfile
            .write_all(b"Bubba")
            .into_watson_result(|| Location::unknown())?;
        let path = tempfile.into_temp_path();

        let mut lexer = Lexer::open(&path)?;
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
    fn lexer_open_path_can_be_overridden() -> Result<()> {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut tempfile = NamedTempFile::new().into_watson_result(|| Location::unknown())?;
        tempfile
            .write_all(b"Bubba")
            .into_watson_result(|| Location::unknown())?;
        let path = tempfile.into_temp_path();
        let path_to_display = path::Path::new("anothername.watson");

        let mut conf = Config::default();
        conf.file_path = Some(path_to_display.to_path_buf().into());
        let mut lexer = Lexer::open_with_config(&path, conf)?;
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
        let mut lexer = Lexer::new(&asciis[..]);
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
        let mut lexer = Lexer::new(&asciis[..]);
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
