use std::mem;

use crate::language::{Insn, Value};
use Insn::*;
use Value::*;

/// A state of a `Serializer`.
#[derive(PartialEq, Clone, Debug)]
enum State {
    //                                                       (n!=0 && n%2==0/None)---> IntWaitingNextBit(n>>1, shamt=shamt+1)
    //                                                                 |
    //                      (*) IntInitial(n) ---(*/Inew)---> IntWaitingNextBit(n, shamt=0) ---(n==0/None)---> Done
    //                                                                 |
    //                                                       (n!=0 && n%2==1/Inew)
    //                                                                 |
    //                                                                 v
    // IntNextBitShifting(n, shamt, i=shamt) <---(*/Iinc)--- IntNextBitAllocated(n, shamt)
    //       |
    //     (i==0/Iadd)---> IntWaitingNextBit(n>>1, shamt=shamt+1)
    //       |
    //     (i!=0/Ishl)---> IntNextBitShifting(n, shamt, i=i-1)
    IntInitial(u64),
    IntWaitingNextBit(u64, usize),
    IntNextBitAllocated(u64, usize),
    IntNextBitShifting(u64, usize, usize),

    // (*) UintInitial(n) ---(*/None)---> UintPushingInt(IntInitial(n)) ---(...)---> UintPushingInt(Done) ---(*/Itou)---> Done
    UintInitial(u64),
    UintPushingInt(Box<State>),

    // Done ---(*/None)---> Done
    Done,
}

impl State {
    /// Returns a new state.
    fn new(v: Value) -> Self {
        match v {
            Int(i) => State::IntInitial(i as u64),
            Uint(u) => State::UintInitial(u),
            _ => todo!(),
        }
    }

    /// Transitions the state and outputs an instruction.
    fn transition(self) -> (State, Option<Insn>) {
        use State::*;

        match self {
            IntInitial(n) => (IntWaitingNextBit(n, 0), Some(Inew)),
            IntWaitingNextBit(n, shamt) => {
                if n == 0 {
                    (Done, None)
                } else if n % 2 == 0 {
                    (IntWaitingNextBit(n >> 1, shamt + 1), None)
                } else {
                    (IntNextBitAllocated(n, shamt), Some(Inew))
                }
            }
            IntNextBitAllocated(n, shamt) => (IntNextBitShifting(n, shamt, shamt), Some(Iinc)),
            IntNextBitShifting(n, shamt, i) => {
                if i == 0 {
                    (IntWaitingNextBit(n >> 1, shamt + 1), Some(Iadd))
                } else {
                    (IntNextBitShifting(n, shamt, i - 1), Some(Ishl))
                }
            }
            UintInitial(n) => (UintPushingInt(Box::new(IntInitial(n))), None),
            UintPushingInt(boxed) => match *boxed {
                Done => (Done, Some(Itou)),
                s => {
                    let (next_s, insn) = s.transition();
                    (UintPushingInt(Box::new(next_s)), insn)
                }
            },
            Done => (Done, None),
        }
    }

    fn take(&mut self) -> State {
        mem::replace(self, State::Done)
    }
}

/// Serializer converts `Value` into a sequence of `Insn`s.
pub struct Serializer {
    state: State,
}

impl Serializer {
    /// Returns a new `Serializer`.
    pub fn new(value: Value) -> Self {
        Serializer {
            state: State::new(value),
        }
    }

    /// Returns the next `Insn` if exists.
    pub fn next_insn(&mut self) -> Option<Insn> {
        let mut prev = self.state.take();
        loop {
            match prev.transition() {
                (State::Done, insn) => {
                    return insn;
                }
                (s, None) => {
                    prev = s;
                }
                (s, Some(insn)) => {
                    self.state = s;
                    return Some(insn);
                }
            }
        }
    }

    /// Returns an iterator over the instructions.
    pub fn into_iter(self) -> IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter(Serializer);

impl Iterator for IntoIter {
    type Item = Insn;

    fn next(&mut self) -> Option<Insn> {
        self.0.next_insn()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::language::{Location, Token};
    use crate::vm;

    #[test]
    fn serializer_int() {
        assert_eq!(to_insn_vec(Int(0)), vec![Inew]);
        assert_eq!(to_insn_vec(Int(1)), vec![Inew, Inew, Iinc, Iadd]);
        assert_eq!(to_insn_vec(Int(2)), vec![Inew, Inew, Iinc, Ishl, Iadd]);
        assert_eq!(
            to_insn_vec(Int(3)),
            vec![Inew, Inew, Iinc, Iadd, Inew, Iinc, Ishl, Iadd]
        );
        assert_eq!(
            to_insn_vec(Int(0b1010101)),
            vec![
                Inew, // 0b0
                Inew, Iinc, Iadd, // 0b1
                Inew, Iinc, Ishl, Ishl, Iadd, // 0b101
                Inew, Iinc, Ishl, Ishl, Ishl, Ishl, Iadd, // 0b10101
                Inew, Iinc, Ishl, Ishl, Ishl, Ishl, Ishl, Ishl, Iadd, // 0b1010101
            ]
        );
        assert_identical(Int(1234567890));
        assert_identical(Int(-1234567890));
    }

    #[test]
    fn serializer_uint() {
        assert_identical(Uint(0));
        assert_identical(Uint(1));
        assert_identical(Uint(5));
        assert_identical(Uint(0xffff_ffff_ffff_ffff));
    }

    /*
     * Helper functions
     */

    fn to_insn_vec(value: Value) -> Vec<Insn> {
        Serializer::new(value).into_iter().collect::<Vec<Insn>>()
    }

    fn assert_identical(value: Value) {
        let orig = value.clone();
        let mut vm = vm::VM::new();
        for insn in Serializer::new(value).into_iter() {
            vm.execute(Token {
                insn: insn,
                location: Location::unknown(),
            })
            .expect("execution error");
        }
        let result = vm.peek_top().expect("stack is empty");
        assert_eq!(&orig, result);
    }
}
