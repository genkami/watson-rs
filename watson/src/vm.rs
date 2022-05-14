use std::path;
use std::rc::Rc;

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

/// A token of the WATSON language.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Token {
    /// A VM instruction that the token represents.
    pub insn: Insn,

    /// An ASCII character that represents the instruction.
    pub ascii: u8,

    /// Location of the instrution.
    pub file_path: Option<Rc<path::Path>>,
    pub line: usize,
    pub column: usize,
}

/// A value that is defined in WATSON specification.
/// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum Value {
    Int(i64),
}

pub use Value::*;

// The error type of the WATSON VM.
#[derive(Eq, PartialEq, Debug)]
pub struct Error {
    /// Represents details of the error.
    pub kind: ErrorKind,

    /// The location where the error happened.
    pub token: Token,
}

/// Details of the `Error`.
#[derive(Eq, PartialEq, Debug)]
pub enum ErrorKind {
    /// The VM tried to pop values from an empty stack.
    EmptyStack,
}

pub type Result<T> = std::result::Result<T, Error>;

/// A stack of the WATSON VM.
/// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
pub struct Stack {
    vec: Vec<Value>,
}

/// StackOps does operations on a stack on behalf of some instruction.
pub struct StackOps<'a> {
    stack: &'a mut Stack,
    token: Token,
}

impl<'a> StackOps<'a> {
    /// Pushes a value onto the stack.
    pub fn push(&mut self, v: Value) {
        self.stack.vec.push(v);
    }

    /// Pops a value from the stack.
    pub fn pop(&mut self) -> Result<Value> {
        match self.stack.vec.pop() {
            Some(x) => Ok(x),
            None => Err(Error {
                kind: ErrorKind::EmptyStack,
                token: self.token.clone(),
            }),
        }
    }
}

impl Stack {
    pub fn new() -> Self {
        Stack { vec: Vec::new() }
    }

    /// Returns a StackOps that can manipulate the stack on behalf of the instruction given by the token.
    pub fn operate_as(&mut self, token: Token) -> StackOps<'_> {
        StackOps {
            stack: self,
            token: token,
        }
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
        let meaningless_token = Token {
            insn: Iadd,
            ascii: b'X',
            file_path: None,
            line: 0,
            column: 0,
        };
        let empty_stack = Err(Error {
            kind: ErrorKind::EmptyStack,
            token: meaningless_token.clone(),
        });

        let mut stack = Stack::new();
        let mut ops = stack.operate_as(meaningless_token);

        assert_eq!(ops.pop(), empty_stack);

        ops.push(Int(1));
        ops.push(Int(2));
        ops.push(Int(3));
        assert_eq!(ops.pop(), Ok(Int(3)));
        assert_eq!(ops.pop(), Ok(Int(2)));
        assert_eq!(ops.pop(), Ok(Int(1)));
        assert_eq!(ops.pop(), empty_stack);
    }
}
