use std::path;
use std::rc::Rc;

/// A value that is defined in WATSON specification.
/// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum Value {
    Int(i64),
    Nil,
}

pub use Value::*;

/// A type that can be converted directly from `Value`.
pub trait FromValue: Sized {
    /// Converts a `Value` into its expected type.
    fn to_inner(v: Value) -> Option<Self>;
}

impl FromValue for i64 {
    fn to_inner(v: Value) -> Option<i64> {
        match v {
            Int(i) => Some(i),
            _ => None,
        }
    }
}

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

    /// The type of the value on the top of stack is different from the one that the instruction wants.
    TypeMismatch,
}

pub type Result<T> = std::result::Result<T, Error>;

/// A stack of the WATSON VM.
/// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
pub struct Stack {
    vec: Vec<Value>,
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

    /// Returns a value on the top of the stack without consuming it.
    pub fn peek_top(&self) -> Option<&Value> {
        let len = self.vec.len();
        if len <= 0 {
            None
        } else {
            Some(&self.vec[len - 1])
        }
    }
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

    /// Pops a value from the stack, applies f to it, then pushes the result.
    pub fn apply1<C1, F>(&mut self, f: F) -> Result<()>
    where
        C1: FromValue,
        F: FnOnce(C1) -> Value,
    {
        let v1 = self.pop()?;
        let result = f(self.claim(v1)?);
        self.push(result);
        Ok(())
    }

    /// Pops two values from the stack, applies f to them, then pushes the result.
    /// The leftmost argument corresponds to the top of the stack.
    pub fn apply2<C1, C2, F>(&mut self, f: F) -> Result<()>
    where
        C1: FromValue,
        C2: FromValue,
        F: FnOnce(C1, C2) -> Value,
    {
        let v1 = self.pop()?;
        let v2 = self.pop()?;
        let result = f(self.claim(v1)?, self.claim(v2)?);
        self.push(result);
        Ok(())
    }

    /// Pops three values from the stack, applies f to them, then pushes the result.
    /// The leftmost argument corresponds to the top of the stack.
    pub fn apply3<C1, C2, C3, F>(&mut self, f: F) -> Result<()>
    where
        C1: FromValue,
        C2: FromValue,
        C3: FromValue,
        F: FnOnce(C1, C2, C3) -> Value,
    {
        let v1 = self.pop()?;
        let v2 = self.pop()?;
        let v3 = self.pop()?;
        let result = f(self.claim(v1)?, self.claim(v2)?, self.claim(v3)?);
        self.push(result);
        Ok(())
    }

    fn claim<C: FromValue>(&self, v: Value) -> Result<C> {
        match C::to_inner(v) {
            Some(x) => Ok(x),
            None => Err(Error {
                kind: ErrorKind::TypeMismatch,
                token: self.token.clone(),
            }),
        }
    }
}

/// A WATSON Virturl Machine.
/// See [the specification](https://github.com/genkami/watson/blob/main/doc/spec.md) for more details.
pub struct VM {
    stack: Stack,
}

impl VM {
    /// Returns a new `VM`.
    pub fn new() -> Self {
        VM {
            stack: Stack::new(),
        }
    }

    /// Executes a single instruction.
    pub fn execute(&mut self, t: Token) -> Result<()> {
        let mut ops = self.stack.operate_as(t.clone());
        // See https://github.com/genkami/watson/blob/main/doc/spec.md#instructions.
        match t.insn {
            Inew => {
                ops.push(Int(0));
                Ok(())
            }
            Iinc => ops.apply1(|x: i64| Int(x + 1)),
            Ishl => ops.apply1(|x: i64| Int(x << 1)),
            Iadd => ops.apply2(|y: i64, x: i64| Int(x + y)),
            Ineg => ops.apply1(|x: i64| Int(-x)),
            Isht => ops.apply2(|y: i64, x: i64| Int(x << y)),
            _ => todo!(),
        }
    }

    /// Returns a `Value` on the top of the stack.
    pub fn peek_top(&self) -> Option<&Value> {
        self.stack.peek_top()
    }

    /// Borrows its stack mutably for debug purpose.
    pub fn borrow_stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stack_push_and_pop() {
        test_ops(|token, mut ops| {
            assert_eq!(ops.pop(), empty_stack(token.clone()));
        });

        test_ops(|token, mut ops| {
            ops.push(Int(1));
            ops.push(Int(2));
            ops.push(Int(3));
            assert_eq!(ops.pop(), Ok(Int(3)));
            assert_eq!(ops.pop(), Ok(Int(2)));
            assert_eq!(ops.pop(), Ok(Int(1)));
            assert_eq!(ops.pop(), empty_stack(token.clone()));
        });
    }

    #[test]
    fn stack_apply1() {
        fn incr(x: i64) -> Value {
            Int(x + 1)
        }

        // insufficient stack
        test_ops(|token, mut ops| {
            assert_eq!(ops.apply1(incr), empty_stack(token.clone()));
        });

        // type mismatch
        test_ops(|token, mut ops| {
            ops.push(Nil);
            assert_eq!(ops.apply1(incr), type_mismatch(token.clone()));
        });

        // ok
        test_ops(|_, mut ops| {
            ops.push(Int(456));
            assert_eq!(ops.apply1(incr), Ok(()));
            assert_eq!(ops.pop(), Ok(Int(457)));
        });
    }

    #[test]
    fn stack_apply2() {
        fn sub(x: i64, y: i64) -> Value {
            Int(x - y)
        }

        // insufficient stack
        test_ops(|token, mut ops| {
            assert_eq!(ops.apply2(sub), empty_stack(token.clone()));
        });

        // insufficient stack (only 1 item)
        test_ops(|token, mut ops| {
            ops.push(Int(5));
            assert_eq!(ops.apply2(sub), empty_stack(token.clone()));
        });

        // type mismatch (first arg)
        test_ops(|token, mut ops| {
            ops.push(Int(5));
            ops.push(Nil);
            assert_eq!(ops.apply2(sub), type_mismatch(token.clone()));
        });

        // type mismatch (second arg)
        test_ops(|token, mut ops| {
            ops.push(Nil);
            ops.push(Int(5));
            assert_eq!(ops.apply2(sub), type_mismatch(token.clone()));
        });

        // ok
        test_ops(|_, mut ops| {
            ops.push(Int(3));
            ops.push(Int(5));
            assert_eq!(ops.apply2(sub), Ok(()));
            assert_eq!(ops.pop(), Ok(Int(2)));
        });
    }

    #[test]
    fn stack_apply3() {
        fn affine(a: i64, x: i64, b: i64) -> Value {
            Int(a * x + b)
        }

        // insufficient stack
        test_ops(|token, mut ops| {
            assert_eq!(ops.apply3(affine), empty_stack(token.clone()));
        });

        // insufficient stack (only 1 item)
        test_ops(|token, mut ops| {
            ops.push(Int(5));
            assert_eq!(ops.apply3(affine), empty_stack(token.clone()));
        });

        // insufficient stack (only 2 items)
        test_ops(|token, mut ops| {
            ops.push(Int(4));
            ops.push(Int(5));
            assert_eq!(ops.apply3(affine), empty_stack(token.clone()));
        });

        // type mismatch (first arg)
        test_ops(|token, mut ops| {
            ops.push(Int(3));
            ops.push(Int(4));
            ops.push(Nil);
            assert_eq!(ops.apply3(affine), type_mismatch(token.clone()));
        });

        // type mismatch (second arg)
        test_ops(|token, mut ops| {
            ops.push(Int(3));
            ops.push(Nil);
            ops.push(Int(5));
            assert_eq!(ops.apply3(affine), type_mismatch(token.clone()));
        });

        // type mismatch (third arg)
        test_ops(|token, mut ops| {
            ops.push(Nil);
            ops.push(Int(4));
            ops.push(Int(5));
            assert_eq!(ops.apply3(affine), type_mismatch(token.clone()));
        });

        // ok
        test_ops(|_, mut ops| {
            ops.push(Int(3));
            ops.push(Int(4));
            ops.push(Int(5));
            assert_eq!(ops.apply3(affine), Ok(()));
            assert_eq!(ops.pop(), Ok(Int(5 * 4 + 3)));
        });
    }

    #[test]
    fn vm_execute_inew() -> Result<()> {
        let mut vm = VM::new();

        assert_eq!(vm.peek_top(), None);
        vm.execute(new_token(Inew))?;
        assert_eq!(vm.peek_top(), Some(&Int(0)));

        Ok(())
    }

    #[test]
    fn vm_execute_iinc() -> Result<()> {
        let mut vm = VM::new();
        let mut ops = vm.borrow_stack_mut().force_operate();

        ops.push(Int(123));
        vm.execute(new_token(Iinc))?;
        assert_eq!(vm.peek_top(), Some(&Int(124)));

        Ok(())
    }

    #[test]
    fn vm_execute_ishl() -> Result<()> {
        let mut vm = VM::new();
        let mut ops = vm.borrow_stack_mut().force_operate();

        ops.push(Int(3));
        vm.execute(new_token(Ishl))?;
        assert_eq!(vm.peek_top(), Some(&Int(6)));

        Ok(())
    }

    #[test]
    fn vm_execute_iadd() -> Result<()> {
        let mut vm = VM::new();
        let mut ops = vm.borrow_stack_mut().force_operate();

        ops.push(Int(3));
        ops.push(Int(4));
        vm.execute(new_token(Iadd))?;
        assert_eq!(vm.peek_top(), Some(&Int(7)));

        Ok(())
    }

    #[test]
    fn vm_execute_ineg() -> Result<()> {
        let mut vm = VM::new();
        let mut ops = vm.borrow_stack_mut().force_operate();

        ops.push(Int(3));
        vm.execute(new_token(Ineg))?;
        assert_eq!(vm.peek_top(), Some(&Int(-3)));

        Ok(())
    }

    #[test]
    fn vm_execute_isht() -> Result<()> {
        let mut vm = VM::new();
        let mut ops = vm.borrow_stack_mut().force_operate();

        ops.push(Int(3));
        ops.push(Int(2));
        vm.execute(new_token(Isht))?;
        assert_eq!(vm.peek_top(), Some(&Int(12)));

        Ok(())
    }

    /*
     * Helper functions
     */

    trait StackExt {
        fn force_operate(&mut self) -> StackOps;
    }

    impl StackExt for Stack {
        fn force_operate(&mut self) -> StackOps {
            self.operate_as(new_meaningless_token())
        }
    }

    fn test_ops<F: FnOnce(Token, StackOps)>(f: F) {
        let token = new_meaningless_token();
        let mut stack = Stack::new();
        let ops = stack.operate_as(token.clone());
        f(token, ops);
    }

    fn new_meaningless_token() -> Token {
        new_token(Iadd)
    }

    fn new_token(insn: Insn) -> Token {
        Token {
            insn: insn,
            ascii: b'X',
            file_path: None,
            line: 0,
            column: 0,
        }
    }

    fn empty_stack<T>(token: Token) -> Result<T> {
        Err(Error {
            kind: ErrorKind::EmptyStack,
            token: token,
        })
    }

    fn type_mismatch<T>(token: Token) -> Result<T> {
        Err(Error {
            kind: ErrorKind::TypeMismatch,
            token: token,
        })
    }
}
