use crate::error::{Error, ErrorKind, Result};
use crate::language::{Insn, IsValue, Map, Token, Value};
use Insn::*;

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
                location: self.token.location.clone(),
                source: None,
            }),
        }
    }

    /// Pops a value from the stack, applies f to it, then pushes the result.
    pub fn apply1<T1, R, F>(&mut self, f: F) -> Result<()>
    where
        T1: IsValue,
        R: IsValue,
        F: FnOnce(T1) -> R,
    {
        let v1 = self.pop()?;
        let result = f(self.claim(v1)?);
        self.push(result.into_value());
        Ok(())
    }

    /// Pops two values from the stack, applies f to them, then pushes the result.
    /// The leftmost argument corresponds to the top of the stack.
    pub fn apply2<T1, T2, R, F>(&mut self, f: F) -> Result<()>
    where
        T1: IsValue,
        T2: IsValue,
        R: IsValue,
        F: FnOnce(T1, T2) -> R,
    {
        let v1 = self.pop()?;
        let v2 = self.pop()?;
        let result = f(self.claim(v1)?, self.claim(v2)?);
        self.push(result.into_value());
        Ok(())
    }

    /// Pops three values from the stack, applies f to them, then pushes the result.
    /// The leftmost argument corresponds to the top of the stack.
    pub fn apply3<T1, T2, T3, R, F>(&mut self, f: F) -> Result<()>
    where
        T1: IsValue,
        T2: IsValue,
        T3: IsValue,
        R: IsValue,
        F: FnOnce(T1, T2, T3) -> R,
    {
        let v1 = self.pop()?;
        let v2 = self.pop()?;
        let v3 = self.pop()?;
        let result = f(self.claim(v1)?, self.claim(v2)?, self.claim(v3)?);
        self.push(result.into_value());
        Ok(())
    }

    fn claim<T: IsValue>(&self, v: Value) -> Result<T> {
        match T::from_value(v) {
            Some(x) => Ok(x),
            None => Err(Error {
                kind: ErrorKind::TypeMismatch,
                location: self.token.location.clone(),
                source: None,
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

        fn push<T: IsValue>(ops: &mut StackOps, x: T) -> Result<()> {
            ops.push(x.into_value());
            Ok(())
        }

        // See https://github.com/genkami/watson/blob/main/doc/spec.md#instructions.
        match t.insn {
            Inew => push(&mut ops, 0_i64),
            Iinc => ops.apply1(|x: i64| x + 1),
            Ishl => ops.apply1(|x: i64| x << 1),
            Iadd => ops.apply2(|y: i64, x: i64| x + y),
            Ineg => ops.apply1(|x: i64| -x),
            Isht => ops.apply2(|y: i64, x: i64| x << y),
            Itof => ops.apply1(|x: i64| f64::from_bits(x as u64)),
            Itou => ops.apply1(|x: i64| x as u64),
            Finf => push(&mut ops, f64::INFINITY),
            Fnan => push(&mut ops, f64::NAN),
            Fneg => ops.apply1(|x: f64| -x),
            Snew => push(&mut ops, Vec::<u8>::new()),
            Sadd => ops.apply2(|x: i64, mut s: Vec<u8>| {
                s.push(x as u8);
                s
            }),
            Onew => push(&mut ops, Map::new()),
            Oadd => ops.apply3(|v: Value, k: Vec<u8>, mut o: Map| {
                o.insert(k, v);
                o
            }),
            Anew => push(&mut ops, Vec::<Value>::new()),
            Aadd => ops.apply2(|v: Value, mut a: Vec<Value>| {
                a.push(v);
                a
            }),
            Bnew => push(&mut ops, false),
            Bneg => ops.apply1(|b: bool| !b),
            Nnew => push(&mut ops, Value::Nil),
            Gdup => {
                let v = ops.pop()?;
                ops.push(v.clone());
                ops.push(v);
                Ok(())
            }
            Gpop => {
                ops.pop()?;
                Ok(())
            }
            Gswp => {
                let v1 = ops.pop()?;
                let v2 = ops.pop()?;
                ops.push(v1);
                ops.push(v2);
                Ok(())
            }
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
    use std::fmt;

    use super::*;
    use crate::language::Location;
    use Value::*;

    #[test]
    fn stack_push_and_pop() -> Result<()> {
        test_ops(|mut ops| {
            assert_error_kind_is(ops.pop(), ErrorKind::EmptyStack);
            Ok(())
        })?;

        test_ops(|mut ops| {
            ops.push(Int(1));
            ops.push(Int(2));
            ops.push(Int(3));
            assert_eq!(ops.pop()?, Int(3));
            assert_eq!(ops.pop()?, Int(2));
            assert_eq!(ops.pop()?, Int(1));
            assert_error_kind_is(ops.pop(), ErrorKind::EmptyStack);
            Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn stack_apply1() -> Result<()> {
        fn incr(x: i64) -> i64 {
            x + 1
        }

        // insufficient stack
        test_ops(|mut ops| {
            assert_error_kind_is(ops.apply1(incr), ErrorKind::EmptyStack);
            Ok(())
        })?;

        // type mismatch
        test_ops(|mut ops| {
            ops.push(Nil);
            assert_error_kind_is(ops.apply1(incr), ErrorKind::TypeMismatch);
            Ok(())
        })?;

        // ok
        test_ops(|mut ops| {
            ops.push(Int(456));
            ops.apply1(incr)?;
            assert_eq!(ops.pop()?, Int(457));
            Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn stack_apply2() -> Result<()> {
        fn sub(x: i64, y: i64) -> i64 {
            x - y
        }

        // insufficient stack
        test_ops(|mut ops| {
            assert_error_kind_is(ops.apply2(sub), ErrorKind::EmptyStack);
            Ok(())
        })?;

        // insufficient stack (only 1 item)
        test_ops(|mut ops| {
            ops.push(Int(5));
            assert_error_kind_is(ops.apply2(sub), ErrorKind::EmptyStack);
            Ok(())
        })?;

        // type mismatch (first arg)
        test_ops(|mut ops| {
            ops.push(Int(5));
            ops.push(Nil);
            assert_error_kind_is(ops.apply2(sub), ErrorKind::TypeMismatch);
            Ok(())
        })?;

        // type mismatch (second arg)
        test_ops(|mut ops| {
            ops.push(Nil);
            ops.push(Int(5));
            assert_error_kind_is(ops.apply2(sub), ErrorKind::TypeMismatch);
            Ok(())
        })?;

        // ok
        test_ops(|mut ops| {
            ops.push(Int(3));
            ops.push(Int(5));
            ops.apply2(sub)?;
            assert_eq!(ops.pop()?, Int(2));
            Ok(())
        })?;

        Ok(())
    }

    #[test]
    fn stack_apply3() -> Result<()> {
        fn affine(a: i64, x: i64, b: i64) -> i64 {
            a * x + b
        }

        // insufficient stack
        test_ops(|mut ops| {
            assert_error_kind_is(ops.apply3(affine), ErrorKind::EmptyStack);
            Ok(())
        })?;

        // insufficient stack (only 1 item)
        test_ops(|mut ops| {
            ops.push(Int(5));
            assert_error_kind_is(ops.apply3(affine), ErrorKind::EmptyStack);
            Ok(())
        })?;

        // insufficient stack (only 2 items)
        test_ops(|mut ops| {
            ops.push(Int(4));
            ops.push(Int(5));
            assert_error_kind_is(ops.apply3(affine), ErrorKind::EmptyStack);
            Ok(())
        })?;

        // type mismatch (first arg)
        test_ops(|mut ops| {
            ops.push(Int(3));
            ops.push(Int(4));
            ops.push(Nil);
            assert_error_kind_is(ops.apply3(affine), ErrorKind::TypeMismatch);
            Ok(())
        })?;

        // type mismatch (second arg)
        test_ops(|mut ops| {
            ops.push(Int(3));
            ops.push(Nil);
            ops.push(Int(5));
            assert_error_kind_is(ops.apply3(affine), ErrorKind::TypeMismatch);
            Ok(())
        })?;

        // type mismatch (third arg)
        test_ops(|mut ops| {
            ops.push(Nil);
            ops.push(Int(4));
            ops.push(Int(5));
            assert_error_kind_is(ops.apply3(affine), ErrorKind::TypeMismatch);
            Ok(())
        })?;

        // ok
        test_ops(|mut ops| {
            ops.push(Int(3));
            ops.push(Int(4));
            ops.push(Int(5));
            ops.apply3(affine)?;
            assert_eq!(ops.pop()?, Int(5 * 4 + 3));
            Ok(())
        })?;

        Ok(())
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

    #[test]
    fn vm_execute_itof() -> Result<()> {
        let mut vm = VM::new();
        let mut ops = vm.borrow_stack_mut().force_operate();
        let f: f64 = 1.234e-56;

        ops.push(Int(f.to_bits() as i64));
        vm.execute(new_token(Itof))?;
        assert_eq!(vm.peek_top(), Some(&Float(f)));

        Ok(())
    }

    #[test]
    fn vm_execute_itou() -> Result<()> {
        let mut vm = VM::new();
        let mut ops = vm.borrow_stack_mut().force_operate();

        ops.push(Int(-1));
        vm.execute(new_token(Itou))?;
        assert_eq!(vm.peek_top(), Some(&Uint(0xffff_ffff_ffff_ffff)));

        Ok(())
    }

    #[test]
    fn vm_execute_finf() -> Result<()> {
        let mut vm = VM::new();

        vm.execute(new_token(Finf))?;
        let top = vm.peek_top().clone();
        let f = match top {
            Some(Float(f)) => f,
            _ => panic!("not a float: {:?}", top),
        };
        assert!(f.is_infinite());
        assert!(f.is_sign_positive());

        Ok(())
    }

    #[test]
    fn vm_execute_fnan() -> Result<()> {
        let mut vm = VM::new();

        vm.execute(new_token(Fnan))?;
        let top = vm.peek_top().clone();
        let f = match top {
            Some(Float(f)) => f,
            _ => panic!("not a float: {:?}", top),
        };
        assert!(f.is_nan());

        Ok(())
    }

    #[test]
    fn vm_execute_fneg() -> Result<()> {
        let mut vm = VM::new();
        let mut ops = vm.borrow_stack_mut().force_operate();

        ops.push(Float(1.23));
        vm.execute(new_token(Fneg))?;
        assert_eq!(vm.peek_top(), Some(&Float(-1.23)));

        Ok(())
    }

    #[test]
    fn vm_execute_snew() -> Result<()> {
        let mut vm = VM::new();

        vm.execute(new_token(Snew))?;
        assert_eq!(vm.peek_top(), Some(&String(Vec::new())));

        Ok(())
    }

    #[test]
    fn vm_execute_sadd() -> Result<()> {
        let mut vm = VM::new();

        vm.execute(new_token(Snew))?;
        assert_eq!(vm.peek_top(), Some(&String(Vec::new())));

        vm.borrow_stack_mut().force_operate().push(Int(b'a' as i64));
        vm.execute(new_token(Sadd))?;
        assert_eq!(vm.peek_top(), Some(&String(b"a".to_vec())));

        vm.borrow_stack_mut().force_operate().push(Int(b'b' as i64));
        vm.execute(new_token(Sadd))?;
        assert_eq!(vm.peek_top(), Some(&String(b"ab".to_vec())));

        Ok(())
    }

    #[test]
    fn vm_execute_onew() -> Result<()> {
        let mut vm = VM::new();

        vm.execute(new_token(Onew))?;
        assert_eq!(vm.peek_top(), Some(&Object(Map::new())));

        Ok(())
    }

    #[test]
    fn vm_execute_oadd() -> Result<()> {
        let mut vm = VM::new();

        vm.execute(new_token(Onew))?;
        assert_eq!(vm.peek_top(), Some(&Object(Map::new())));

        let mut ops = vm.borrow_stack_mut().force_operate();
        ops.push(String(b"key1".to_vec()));
        ops.push(String(b"value1".to_vec()));
        vm.execute(new_token(Oadd))?;
        assert_eq!(
            vm.peek_top(),
            Some(&Object(
                vec![(b"key1".to_vec(), String(b"value1".to_vec()))]
                    .into_iter()
                    .collect()
            ))
        );

        let mut ops = vm.borrow_stack_mut().force_operate();
        ops.push(String(b"key2".to_vec()));
        ops.push(Int(22222));
        vm.execute(new_token(Oadd))?;
        assert_eq!(
            vm.peek_top(),
            Some(&Object(
                vec![
                    (b"key1".to_vec(), String(b"value1".to_vec())),
                    (b"key2".to_vec(), Int(22222)),
                ]
                .into_iter()
                .collect()
            ))
        );
        Ok(())
    }

    #[test]
    fn vm_execute_anew() -> Result<()> {
        let mut vm = VM::new();

        vm.execute(new_token(Anew))?;
        assert_eq!(vm.peek_top(), Some(&Array(Vec::new())));

        Ok(())
    }

    #[test]
    fn vm_execute_aadd() -> Result<()> {
        let mut vm = VM::new();

        vm.execute(new_token(Anew))?;
        assert_eq!(vm.peek_top(), Some(&Array(Vec::new())));

        vm.borrow_stack_mut().force_operate().push(Int(123));
        vm.execute(new_token(Aadd))?;
        assert_eq!(vm.peek_top(), Some(&Array(vec![Int(123)])));

        vm.borrow_stack_mut()
            .force_operate()
            .push(String(b"hello".to_vec()));
        vm.execute(new_token(Aadd))?;
        assert_eq!(
            vm.peek_top(),
            Some(&Array(vec![Int(123), String(b"hello".to_vec())]))
        );

        Ok(())
    }

    #[test]
    fn vm_execute_bnew() -> Result<()> {
        let mut vm = VM::new();

        vm.execute(new_token(Bnew))?;
        assert_eq!(vm.peek_top(), Some(&Bool(false)));

        Ok(())
    }

    #[test]
    fn vm_execute_bneg() -> Result<()> {
        let mut vm = VM::new();

        vm.execute(new_token(Bnew))?;
        assert_eq!(vm.peek_top(), Some(&Bool(false)));

        vm.execute(new_token(Bneg))?;
        assert_eq!(vm.peek_top(), Some(&Bool(true)));

        vm.execute(new_token(Bneg))?;
        assert_eq!(vm.peek_top(), Some(&Bool(false)));
        Ok(())
    }

    #[test]
    fn vm_execute_nnew() -> Result<()> {
        let mut vm = VM::new();

        vm.execute(new_token(Nnew))?;
        assert_eq!(vm.peek_top(), Some(&Nil));

        Ok(())
    }

    #[test]
    fn vm_execute_gdup() -> Result<()> {
        let mut vm = VM::new();

        vm.borrow_stack_mut().force_operate().push(Int(123));
        vm.execute(new_token(Gdup))?;

        let mut ops = vm.borrow_stack_mut().force_operate();
        assert_eq!(ops.pop()?, Int(123));
        assert_eq!(ops.pop()?, Int(123));
        assert_error_kind_is(ops.pop(), ErrorKind::EmptyStack);

        Ok(())
    }

    #[test]
    fn vm_execute_gpop() -> Result<()> {
        let mut vm = VM::new();
        let mut ops = vm.borrow_stack_mut().force_operate();

        ops.push(Int(111));
        ops.push(Uint(222));
        vm.execute(new_token(Gpop))?;

        let mut ops = vm.borrow_stack_mut().force_operate();
        assert_eq!(ops.pop()?, Int(111));
        assert_error_kind_is(ops.pop(), ErrorKind::EmptyStack);

        Ok(())
    }

    #[test]
    fn vm_execute_gswp() -> Result<()> {
        let mut vm = VM::new();
        let mut ops = vm.borrow_stack_mut().force_operate();

        ops.push(Int(111));
        ops.push(Uint(222));
        vm.execute(new_token(Gswp))?;

        let mut ops = vm.borrow_stack_mut().force_operate();
        assert_eq!(ops.pop()?, Int(111));
        assert_eq!(ops.pop()?, Uint(222));
        assert_error_kind_is(ops.pop(), ErrorKind::EmptyStack);

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

    fn test_ops<F: FnOnce(StackOps) -> Result<()>>(f: F) -> Result<()> {
        let mut stack = Stack::new();
        f(stack.force_operate())
    }

    fn new_meaningless_token() -> Token {
        new_token(Iadd)
    }

    fn new_token(insn: Insn) -> Token {
        Token {
            insn: insn,
            location: Location {
                ascii: b'X',
                path: None,
                line: 0,
                column: 0,
            },
        }
    }

    fn assert_error_kind_is<T: fmt::Debug>(res: Result<T>, kind: ErrorKind) {
        assert_eq!(res.unwrap_err().kind, kind);
    }
}
