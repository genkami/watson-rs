/// An instruction of the WATSON Virtual Machine.
/// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum Insn {
    Inew,
    Iinc,
    Ishl,
    Iadd,
    Ineg,
    Isht,
    Itof,
    Itou,
    Finf,
    Fnan,
    Fneg,
    Snew,
    Sadd,
    Onew,
    Oadd,
    Anew,
    Aadd,
    Bnew,
    Bneg,
    Nnew,
    Gdup,
    Gpop,
    Gswp,
}

use Insn::*;

pub const ALL_INSNS: [Insn; 23] = [
    Inew, Iinc, Ishl, Iadd, Ineg, Isht, Itof, Itou, Finf, Fnan, Fneg, Snew, Sadd, Onew, Oadd, Anew,
    Aadd, Bnew, Bneg, Nnew, Gdup, Gpop, Gswp,
];
