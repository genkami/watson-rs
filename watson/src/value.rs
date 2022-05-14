/// A value that is defined in WATSON specification.
/// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum Value {
    Int(i64),
    Nil,
}

pub use Value::*;

/// Claimer claims that a Value of certain type should be lying on the top of the stack.
pub trait Claimer {
    /// The expected type of a `Value`.
    type Want;

    /// Converts a `Value` into its expected type.
    fn to_inner(x: Value) -> Option<Self::Want>;
}

pub mod claimer {
    use super::*;

    pub struct Int;

    impl Claimer for Int {
        type Want = i64;

        fn to_inner(x: Value) -> Option<i64> {
            match x {
                Value::Int(i) => Some(i),
                _ => None,
            }
        }
    }
}
