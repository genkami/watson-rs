use std::fs;
use std::io;
use std::path;

use crate::error::Result;
use crate::language::{Insn, Mode};

const DEFAULT_CHARS_PER_LINE: usize = 80;

/// `Unlexer` converts a sequence of `Insn`s to its ASCII representation.
pub struct Unlexer<W: io::Write> {
    writer: W,

    mode: Mode,
    chars_per_line: usize,

    column: usize,
}

/// Config configures an `Unlexer`.
pub struct Config {
    /// Initial mode of an `Unlexer` (defaults to `A` by the specification).
    pub initial_mode: Mode,

    /// An `Unlexer` emits a newline character every time it emits `chars_per_line` consecutive characters.
    /// If set to zero, then `Unlexer` does not emit any newline characters.
    pub chars_per_line: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            initial_mode: Mode::A,
            chars_per_line: DEFAULT_CHARS_PER_LINE,
        }
    }
}

impl Config {
    /// Returns a new `Unlexer` that writes to the given writer.
    pub fn new<W: io::Write>(self, writer: W) -> Unlexer<W> {
        Unlexer {
            writer: writer,
            mode: self.initial_mode,
            chars_per_line: self.chars_per_line,
            column: 0,
        }
    }

    /// Creates a file (by `fs::File::create`) and returns an `Unlexer` that writes to this file.
    pub fn open(self, path: &path::Path) -> Result<Unlexer<fs::File>> {
        let f = fs::File::create(path)?;
        Ok(self.new(f))
    }
}

impl Unlexer<fs::File> {
    /// Creates a file (by `fs::File::create`) and returns an `Unlexer` that writes to this file with the default configuration.
    pub fn open(path: &path::Path) -> Result<Self> {
        Config::default().open(path)
    }
}

impl<W: io::Write> Unlexer<W> {
    /// Returns a new `Unlexer` that writes to the given writer with the default configuration.
    pub fn new(writer: W) -> Self {
        Config::default().new(writer)
    }

    /// Writes a single `Insn` to its underlying writer.
    pub fn write(&mut self, insn: Insn) -> Result<()> {
        let mut buf = [self.mode.insn_to_ascii(insn)];
        self.writer.write_all(&buf)?;
        self.column += 1;
        if 0 < self.chars_per_line && self.chars_per_line <= self.column {
            self.column = 0;
            buf[0] = b'\n';
            self.writer.write_all(&buf)?;
        }
        match insn {
            Insn::Snew => {
                self.mode = self.mode.flip();
            }
            _ => {
                // nop
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use Insn::*;

    #[test]
    fn unlexer_new_initial_mode_defaults_to_a() -> Result<()> {
        let mut buf = Vec::new();
        let mut unlexer = Unlexer::new(&mut buf);
        unlexer.write(Insn::Inew)?;
        assert_eq!(buf, b"B".to_vec());
        Ok(())
    }

    #[test]
    fn unlexer_new_initlal_mode_is_configurable() -> Result<()> {
        let mut conf = Config::default();
        conf.initial_mode = Mode::S;
        let mut buf = Vec::new();
        let mut unlexer = conf.new(&mut buf);
        unlexer.write(Insn::Inew)?;
        assert_eq!(buf, b"S".to_vec());
        Ok(())
    }

    #[test]
    fn unlexer_open_opens_a_file() -> Result<()> {
        use io::BufRead;

        let tempdir = tempfile::tempdir()?;
        let path = tempdir.path().join("data.watson");
        {
            let mut unlexer = Unlexer::open(&path)?;
            unlexer.write(Insn::Inew)?;
        }

        let mut lines = io::BufReader::new(fs::File::open(&path)?).lines();
        assert_eq!(lines.next().unwrap().unwrap(), "B");
        assert!(lines.next().is_none());

        Ok(())
    }

    #[test]
    fn unlexer_changes_its_mode() -> Result<()> {
        let mut buf = Vec::new();
        let mut unlexer = Unlexer::new(&mut buf);
        for insn in vec![Inew, Snew, Inew, Snew, Inew] {
            unlexer.write(insn)?;
        }
        assert_eq!(buf, b"B?S$B".to_vec());
        Ok(())
    }

    #[test]
    fn unlexer_emits_newline() -> Result<()> {
        let mut conf = Config::default();
        conf.chars_per_line = 5;
        let mut buf = Vec::new();
        let mut unlexer = conf.new(&mut buf);

        for insn in vec![
            Inew, Iinc, Ishl, Ishl, Iadd, Snew, Inew, Iinc, Ishl, Ishl, Iadd, Snew, Inew,
        ] {
            unlexer.write(insn)?;
        }
        assert_eq!(buf, b"Bubba\n?Shaa\nk$B".to_vec());

        Ok(())
    }
}
