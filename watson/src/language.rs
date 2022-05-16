use std::path;
use std::rc::Rc;

/// A value that is defined in WATSON specification.
/// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Int(i64),
    Uint(u64),
    Float(f64),
    String(Vec<u8>),
    Object(Map),
    Array(Vec<Value>),
    Bool(bool),
    Nil,
}

use Value::*;

/// A type corresponding to WATSON Object.
pub type Map = std::collections::HashMap<Vec<u8>, Value>;

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

/// A token of the WATSON language.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Token {
    /// A VM instruction that the token represents.
    pub insn: Insn,

    /// Location of the instruction.
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

impl Location {
    pub fn unknown() -> Self {
        Location {
            ascii: 0,
            path: None,
            line: 0,
            column: 0,
        }
    }
}

/// A "mode" of the WATSON lexer.
/// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
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
    pub fn ascii_to_insn(self, ascii: u8) -> Option<Insn> {
        let table = match self {
            Mode::A => &*conv::ASCII_TO_INSN_TABLE_A,
            Mode::S => &*conv::ASCII_TO_INSN_TABLE_S,
        };
        table.get(&ascii).map(|i| *i)
    }

    // Converts a `vm::Insn` to its corresponding ASCII character with respect to the current `Mode`.
    pub fn insn_to_ascii(self, insn: Insn) -> u8 {
        let table = match self {
            Mode::A => &*conv::INSN_TO_ASCII_TABLE_A,
            Mode::S => &*conv::INSN_TO_ASCII_TABLE_S,
        };
        table.get(&insn).map(|c| *c).unwrap()
    }
}

macro_rules! impl_from_int_for_value {
    ( $( $t:ty ),* ) => {
        $(
            impl From<$t> for Value {
                fn from(v: $t) -> Value {
                    Int(v as i64)
                }
            }
        )*
    };
}

impl_from_int_for_value!(i8, i16, i32, i64, i128, isize);

macro_rules! impl_from_uint_for_value {
    ( $( $t:ty ),* ) => {
        $(
            impl From<$t> for Value {
                fn from(v: $t) -> Value {
                    Uint(v as u64)
                }
            }
        )*
    };
}

impl_from_uint_for_value!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_from_float_for_value {
    ( $( $t:ty ),* ) => {
        $(
            impl From<$t> for Value {
                fn from(v: $t) -> Value {
                    Float(v as f64)
                }
            }
        )*
    };
}

impl_from_float_for_value!(f32, f64);

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Value {
        String(v)
    }
}

impl From<Map> for Value {
    fn from(v: Map) -> Value {
        Object(v)
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Value {
        Array(v)
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Value {
        Bool(v)
    }
}

/// A type that can be converted directly from and to `Value`.
/// This is different from From<Value> and Into<Value> in that the values of these these types are "identical" to `Value`.
pub trait IsValue: Into<Value> {
    /// Converts a `Value` into its expected type.
    fn from_value(v: Value) -> Option<Self>;

    /// Converts self into a `Value`.
    fn into_value(self) -> Value {
        self.into()
    }
}

impl IsValue for Value {
    fn from_value(v: Value) -> Option<Value> {
        Some(v)
    }
}

impl IsValue for i64 {
    fn from_value(v: Value) -> Option<i64> {
        match v {
            Int(i) => Some(i),
            _ => None,
        }
    }
}

impl IsValue for u64 {
    fn from_value(v: Value) -> Option<u64> {
        match v {
            Uint(u) => Some(u),
            _ => None,
        }
    }
}

impl IsValue for f64 {
    fn from_value(v: Value) -> Option<f64> {
        match v {
            Float(f) => Some(f),
            _ => None,
        }
    }
}

impl IsValue for Vec<u8> {
    fn from_value(v: Value) -> Option<Vec<u8>> {
        match v {
            String(s) => Some(s),
            _ => None,
        }
    }
}

impl IsValue for Map {
    fn from_value(v: Value) -> Option<Map> {
        match v {
            Object(o) => Some(o),
            _ => None,
        }
    }
}

impl IsValue for Vec<Value> {
    fn from_value(v: Value) -> Option<Vec<Value>> {
        match v {
            Array(a) => Some(a),
            _ => None,
        }
    }
}

impl IsValue for bool {
    fn from_value(v: Value) -> Option<bool> {
        match v {
            Bool(b) => Some(b),
            _ => None,
        }
    }
}

mod conv {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    use super::*;

    use Insn::*;

    pub const ALL_INSNS: [Insn; 23] = [
        Inew, Iinc, Ishl, Iadd, Ineg, Isht, Itof, Itou, Finf, Fnan, Fneg, Snew, Sadd, Onew, Oadd,
        Anew, Aadd, Bnew, Bneg, Nnew, Gdup, Gpop, Gswp,
    ];

    // See https://github.com/genkami/watson/blob/main/doc/spec.md#watson-representation.
    pub static ASCII_TO_INSN_TABLE_A: Lazy<HashMap<u8, Insn>> =
        Lazy::new(|| build_ascii_to_insn_map(b"BubaAei'qtp?!~M@szo.E#%"));

    pub static ASCII_TO_INSN_TABLE_S: Lazy<HashMap<u8, Insn>> =
        Lazy::new(|| build_ascii_to_insn_map(b"ShakrAzimbu$-+gv?^!y/e:"));

    pub static INSN_TO_ASCII_TABLE_A: Lazy<HashMap<Insn, u8>> =
        Lazy::new(|| reverse(&*ASCII_TO_INSN_TABLE_A));

    pub static INSN_TO_ASCII_TABLE_S: Lazy<HashMap<Insn, u8>> =
        Lazy::new(|| reverse(&*ASCII_TO_INSN_TABLE_S));

    fn build_ascii_to_insn_map(asciis: &[u8]) -> HashMap<u8, Insn> {
        asciis
            .iter()
            .zip(&ALL_INSNS)
            .map(|(c, i)| (*c, *i))
            .collect()
    }

    fn reverse(orig: &HashMap<u8, Insn>) -> HashMap<Insn, u8> {
        orig.iter().map(|(c, i)| (*i, *c)).collect()
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

            let mut insns = conv::ALL_INSNS.iter().map(|i| *i).collect::<HashSet<_>>();
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
            for i in conv::ALL_INSNS {
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
            for i in conv::ALL_INSNS {
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
