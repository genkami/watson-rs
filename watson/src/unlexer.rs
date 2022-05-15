use std::io;

/// `Unlexer` converts a sequence of `Insn`s to its ASCII representation.
struct Unlexer<W: io::Write> {
    writer: W,
}
