use std::fmt;
use std::path;
use std::rc::Rc;

mod conversion;

pub use self::conversion::{IsValue, ToBytes};

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

            /// Converts a byte representation into corresponding `Insn`.
            /// Which byte is converted to which insn depends on `Mode`.
            pub fn from_byte(mode: Mode, byte: u8) -> Option<Self> {
                match mode {
                    Mode::A => Insn::from_byte_a(byte),
                    Mode::S => Insn::from_byte_s(byte),
                }
            }

            /// Converts itself into its byte representation.
            /// Which insn is converted to which byte depends on `Mode`.
            pub fn into_byte(self, mode: Mode) -> u8 {
                match mode {
                    Mode::A => self.into_byte_a(),
                    Mode::S => self.into_byte_s(),
                }
            }

            fn from_byte_a(byte: u8) -> Option<Self> {
                match byte {
                    $(
                        $achar => Some(Insn::$name),
                    )*
                    _ => None,
                }
            }

            fn from_byte_s(byte: u8) -> Option<Self> {
                match byte {
                    $(
                        $schar => Some(Insn::$name),
                    )*
                    _ => None,
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

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.path.as_ref() {
            Some(p) => {
                write!(f, "{}", p.to_string_lossy())?;
            }
            None => {
                write!(f, "unknown file")?;
            }
        }
        write!(f, " (line: {}, column: {})", self.line, self.column)?;
        if let Some(c) = char::from_u32(self.byte as u32) {
            write!(f, ", near the character {c}")?;
        }
        Ok(())
    }
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
}

/// Creates an array `Value` consisting of the arguments.
#[macro_export]
macro_rules! array {
    // To suppress unused_mut.
    () => {
        $crate::language::Value::Array(std::vec::Vec::new())
    };
    ( $( $elem:expr ),* $(,)? ) => {{
        let mut vec = std::vec::Vec::new();
        $(
            vec.push($elem);
        )*
        $crate::language::Value::Array(vec)
    }}
}

#[macro_export]
#[doc(hidden)]
macro_rules! object_key {
    ($key:ident) => {
        $crate::language::ToBytes::to_bytes(stringify!($key))
    };
    ([ $key:expr ]) => {
        $crate::language::ToBytes::to_bytes($key)
    };
}

/// Creates an object `Value` consisting of the given key-value pairs.
/// The key must be an identifier or an expression that implements `ToBytes` surrounded by `[` and `]`.
/// The value must be any expression of type `Value`.
#[macro_export]
macro_rules! object {
    // To suppress unused_mut.
    () => {
        $crate::language::Value::Object($crate::language::Map::new())
    };
    ( $( $key:tt : $value:expr ),* $(,)? ) => {{
        let mut map = $crate::language::Map::new();
        $(
            map.insert($crate::object_key!($key), $value);
        )*
        $crate::language::Value::Object(map)
    }};
}

#[cfg(test)]
mod test {
    use super::*;
    use Value::*;

    // 0x21 to 0x7E
    const ASCII_CHARS: std::ops::RangeInclusive<u8> = b'!'..=b'~';

    #[test]
    fn insn_from_byte_is_surjective() {
        fn assert_surjective(mode: Mode) {
            use std::collections::HashSet;

            let mut insns = Insn::all().collect::<HashSet<_>>();
            for c in ASCII_CHARS {
                Insn::from_byte(mode, c).map(|insn| insns.remove(&insn));
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
    fn insn_from_byte_is_injective() {
        fn assert_injective(mode: Mode) {
            use std::collections::HashMap;

            let mut reversed = HashMap::new();
            for c in ASCII_CHARS {
                Insn::from_byte(mode, c).map(|insn| match reversed.get(&insn) {
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
    fn insn_into_byte_is_injective() {
        fn assert_injective(mode: Mode) {
            use std::collections::HashMap;

            let mut reversed = HashMap::new();
            for i in Insn::all() {
                let c = i.into_byte(mode);
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

    #[test]
    fn array_macro() {
        assert_eq!(array![], Array(vec![]));
        assert_eq!(array![Int(123)], Array(vec![Int(123)]));
        assert_eq!(
            array![Int(123), Bool(false), array![Uint(456)]],
            Array(vec![Int(123), Bool(false), Array(vec![Uint(456)])])
        );
        assert_eq!(
            array![
                Int(123),
                Bool(false),
                array![Uint(456)], // trailing comma
            ],
            Array(vec![Int(123), Bool(false), Array(vec![Uint(456)])])
        )
    }

    #[test]
    fn object_macro() {
        assert_eq!(object![], Object([].into_iter().collect()));
        assert_eq!(
            object![x: Int(1)],
            Object([(b"x".to_vec(), Int(1))].into_iter().collect())
        );
        assert_eq!(
            object![[b"y"]: Int(1)],
            Object([(b"y".to_vec(), Int(1))].into_iter().collect())
        );
        assert_eq!(
            object![x: Int(1), y: Bool(true), ['ぬ']: object![nested: Nil]],
            Object(
                [
                    (b"x".to_vec(), Int(1)),
                    (b"y".to_vec(), Bool(true)),
                    (
                        "ぬ".to_string().into_bytes(),
                        Object([(b"nested".to_vec(), Nil)].into_iter().collect())
                    )
                ]
                .into_iter()
                .collect()
            )
        );
        assert_eq!(
            object![
                x: Int(1), y: Bool(true), z: object![nested: Nil], // trailing comma
            ],
            Object(
                [
                    (b"x".to_vec(), Int(1)),
                    (b"y".to_vec(), Bool(true)),
                    (
                        b"z".to_vec(),
                        Object([(b"nested".to_vec(), Nil)].into_iter().collect())
                    )
                ]
                .into_iter()
                .collect()
            )
        );
    }
}
