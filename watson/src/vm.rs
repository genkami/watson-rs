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

pub use Insn::*;

/// A value that is defined in WATSON specification.
/// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum Value {
    Int(i64),
}

pub use Value::*;

/// A stack of the WATSON VM.
/// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
pub struct Stack {
    vec: Vec<Value>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { vec: Vec::new() }
    }

    /// Pushes a value onto the stack.
    pub fn push(&mut self, v: Value) {
        self.vec.push(v);
    }

    /// Pops a value from the stack.
    pub fn pop(&mut self) -> Option<Value> {
        self.vec.pop()
    }
}

/// A WATSON Virturl Machine.
/// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
pub struct VM {
    stack: Stack,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stack_push_and_pop() {
        let mut s = Stack::new();
        assert_eq!(s.pop(), None);

        s.push(Int(1));
        s.push(Int(2));
        s.push(Int(3));
        assert_eq!(s.pop(), Some(Int(3)));
        assert_eq!(s.pop(), Some(Int(2)));
        assert_eq!(s.pop(), Some(Int(1)));
        assert_eq!(s.pop(), None);
    }
}
