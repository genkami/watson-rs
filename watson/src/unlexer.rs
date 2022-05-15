use std::fs;
use std::io;
use std::path;

use crate::error::Result;
use crate::language::{Insn, Mode};

const CHARS_PER_LINE: usize = 80;

/// `Unlexer` converts a sequence of `Insn`s to its ASCII representation.
pub struct Unlexer<W: io::Write> {
    writer: W,

    mode: Mode,
    compact: bool,

    column: usize,
}

/// Config configures an `Unlexer`.
pub struct Config {
    /// Initial mode of an `Unlexer` (defaults to `A` by the specification).
    pub initial_mode: Mode,

    /// If set to true, `Unlexer` does not write whitespaces.
    pub compact: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            initial_mode: Mode::A,
            compact: false,
        }
    }
}

impl Config {
    /// Returns a new `Unlexer` that writes to the given writer.
    pub fn new<W: io::Write>(self, writer: W) -> Unlexer<W> {
        Unlexer {
            writer: writer,
            mode: self.initial_mode,
            compact: self.compact,
            column: 0,
        }
    }

    /// Creates a file (by `fs::File::create`) and returns an `Unlexer` that writes to this file.
    pub fn open(self, path: &path::Path) -> Result<Unlexer<fs::File>> {
        let f = fs::File::open(path)?;
        Ok(self.new(f))
    }
}

impl Unlexer<fs::File> {
    /// Creates a file (by `fs::File::create`) and returns an `Unlexer` that writes to this file with the default configuration.
    pub fn open(self, path: &path::Path) -> Result<Self> {
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
        if !self.compact && CHARS_PER_LINE <= self.column {
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

    #[test]
    fn unlexer_new_initial_mode_defaults_to_a() -> Result<()> {
        let mut buf = VecWriter::new();
        let mut unlexer = Unlexer::new(&mut buf);
        unlexer.write(Insn::Inew)?;
        assert_eq!(buf.into_vec(), b"B".to_vec());
        Ok(())
    }

    #[test]
    fn unlexer_new_initlal_mode_is_configurable() -> Result<()> {
        let mut conf = Config::default();
        conf.initial_mode = Mode::S;
        let mut buf = VecWriter::new();
        let mut unlexer = conf.new(&mut buf);
        unlexer.write(Insn::Inew)?;
        assert_eq!(buf.into_vec(), b"S".to_vec());
        Ok(())
    }

    struct VecWriter(Vec<u8>);

    impl VecWriter {
        fn new() -> Self {
            VecWriter(Vec::new())
        }

        fn into_vec(self) -> Vec<u8> {
            self.0
        }
    }

    impl<'a> io::Write for &'a mut VecWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            for b in buf.iter() {
                self.0.push(*b);
            }
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
}
