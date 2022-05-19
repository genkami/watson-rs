use std::path;
use std::rc::Rc;

/// A byte array.
pub type Bytes = Vec<u8>;

/// A type corresponding to WATSON Object.
pub type Map = std::collections::HashMap<Bytes, Value>;

/// A value that is defined in WATSON specification.
/// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    Int(i64),
    Uint(u64),
    Float(f64),
    String(Bytes),
    Object(Map),
    Array(Vec<Value>),
    Bool(bool),
    Nil,
}

use Value::*;

macro_rules! define_insn {
    ( $( ($name:ident, $achar:expr, $schar:expr) ),* ) => {
        /// An instruction of the WATSON Virtual Machine.
        /// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
        #[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
        pub enum Insn {
            $( $name ),*
        }

        impl Insn {
            /// Returns an iterator that iterates over all instructions.
            pub fn all() -> impl Iterator<Item = Self> {
                [$( Insn::$name ),* ].into_iter()
            }

            /// Converts itself into its byte representation.
            pub fn into_byte(self, mode: Mode) -> u8 {
                match mode {
                    Mode::A => self.into_byte_a(),
                    Mode::S => self.into_byte_s(),
                }
            }

            fn into_byte_a(self) -> u8 {
                match self {
                    $(
                        Insn::$name => $achar
                    ),*
                }
            }

            fn into_byte_s(self) -> u8 {
                match self {
                    $(
                        Insn::$name => $schar
                    ),*
                }
            }
        }
    };
    ( $( ($name:ident, $achar:expr, $schar:expr) ),* ,) => {
        define_insn!( $( ($name, $achar, $schar) ),* );
    }
}

define_insn! {
    (Inew, b'B', b'S'),
    (Iinc, b'u', b'h'),
    (Ishl, b'b', b'a'),
    (Iadd, b'a', b'k'),
    (Ineg, b'A', b'r'),
    (Isht, b'e', b'A'),
    (Itof, b'i', b'z'),
    (Itou, b'\'', b'i'),
    (Finf, b'q', b'm'),
    (Fnan, b't', b'b'),
    (Fneg, b'p', b'u'),
    (Snew, b'?', b'$'),
    (Sadd, b'!', b'-'),
    (Onew, b'~', b'+'),
    (Oadd, b'M', b'g'),
    (Anew, b'@', b'v'),
    (Aadd, b's', b'?'),
    (Bnew, b'z', b'^'),
    (Bneg, b'o', b'!'),
    (Nnew, b'.', b'y'),
    (Gdup, b'E', b'/'),
    (Gpop, b'#', b'e'),
    (Gswp, b'%', b':'),
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
    /// A byte that the WATSON VM read.
    pub byte: u8,

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
            byte: 0,
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

    // Converts an byte character to its corresponding `vm::Insn` with respect to the current `Mode`.
    pub fn byte_to_insn(self, byte: u8) -> Option<Insn> {
        let table = match self {
            Mode::A => &*conv::BYTE_TO_INSN_TABLE_A,
            Mode::S => &*conv::BYTE_TO_INSN_TABLE_S,
        };
        table.get(&byte).map(|i| *i)
    }

    // Converts a `vm::Insn` to its corresponding byte character with respect to the current `Mode`.
    pub fn insn_to_byte(self, insn: Insn) -> u8 {
        let table = match self {
            Mode::A => &*conv::INSN_TO_BYTE_TABLE_A,
            Mode::S => &*conv::INSN_TO_BYTE_TABLE_S,
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

impl From<Bytes> for Value {
    fn from(v: Bytes) -> Value {
        String(v)
    }
}

impl From<std::string::String> for Value {
    fn from(v: std::string::String) -> Value {
        String(v.into_bytes())
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

impl IsValue for Bytes {
    fn from_value(v: Value) -> Option<Bytes> {
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

/// A type that can be converted to `Bytes`.
pub trait ToBytes {
    /// Converts `self` to `Bytes`.
    fn to_bytes(&self) -> Bytes;
}

impl ToBytes for Bytes {
    fn to_bytes(&self) -> Bytes {
        self.clone()
    }
}

impl ToBytes for &Bytes {
    fn to_bytes(&self) -> Bytes {
        self.to_vec()
    }
}

impl ToBytes for std::string::String {
    fn to_bytes(&self) -> Bytes {
        self.to_owned().into_bytes()
    }
}

impl ToBytes for &str {
    fn to_bytes(&self) -> Bytes {
        self.as_bytes().to_vec()
    }
}

mod conv {
    use once_cell::sync::Lazy;
    use std::collections::HashMap;

    use super::*;

    // See https://github.com/genkami/watson/blob/main/doc/spec.md#watson-representation.
    pub static BYTE_TO_INSN_TABLE_A: Lazy<HashMap<u8, Insn>> =
        Lazy::new(|| build_byte_to_insn_map(b"BubaAei'qtp?!~M@szo.E#%"));

    pub static BYTE_TO_INSN_TABLE_S: Lazy<HashMap<u8, Insn>> =
        Lazy::new(|| build_byte_to_insn_map(b"ShakrAzimbu$-+gv?^!y/e:"));

    pub static INSN_TO_BYTE_TABLE_A: Lazy<HashMap<Insn, u8>> =
        Lazy::new(|| reverse(&*BYTE_TO_INSN_TABLE_A));

    pub static INSN_TO_BYTE_TABLE_S: Lazy<HashMap<Insn, u8>> =
        Lazy::new(|| reverse(&*BYTE_TO_INSN_TABLE_S));

    fn build_byte_to_insn_map(bytes: &[u8]) -> HashMap<u8, Insn> {
        bytes
            .iter()
            .zip(Insn::all())
            .map(|(c, i)| (*c, i))
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
    fn mode_byte_to_insn_is_surjective() {
        fn assert_surjective(mode: Mode) {
            use std::collections::HashSet;

            let mut insns = Insn::all().collect::<HashSet<_>>();
            for c in ASCII_CHARS {
                mode.byte_to_insn(c).map(|insn| insns.remove(&insn));
            }
            for insn in insns {
                panic!(
                    "mode={:?}: instruction {:?} does not have matching byte characters",
                    mode, insn
                );
            }
        }

        assert_surjective(Mode::A);
        assert_surjective(Mode::S);
    }

    #[test]
    fn mode_byte_to_insn_is_injective() {
        fn assert_injective(mode: Mode) {
            use std::collections::HashMap;

            let mut reversed = HashMap::new();
            for c in ASCII_CHARS {
                mode.byte_to_insn(c).map(|insn| match reversed.get(&insn) {
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
    fn mode_insn_to_byte_never_panics() {
        fn assert_never_panics(mode: Mode) {
            for i in Insn::all() {
                mode.insn_to_byte(i);
            }
        }

        assert_never_panics(Mode::A);
        assert_never_panics(Mode::S);
    }

    #[test]
    fn mode_insn_to_byte_is_injective() {
        fn assert_injective(mode: Mode) {
            use std::collections::HashMap;

            let mut reversed = HashMap::new();
            for i in Insn::all() {
                let c = mode.insn_to_byte(i);
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
