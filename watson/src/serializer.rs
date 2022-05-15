use std::mem;

use crate::language::{Insn, Map, Value};
use Insn::*;
use Value::*;

type MapIter = std::collections::hash_map::IntoIter<Vec<u8>, Value>;
type ArrayIter = std::vec::IntoIter<Value>;

/// A state of a `Serializer`.
#[derive(Debug)]
enum State {
    // (*) IntInitial(n)
    //         |
    //      (*/Inew)
    //         |
    //         v
    //     IntWaitingNextBit(n, shamt=0) ---(n==0/None)---> Done
    //         |
    //      (n!=0 && n%2==0/None)---> IntWaitingNextBit(n>>1, shamt=shamt+1)
    //         |
    //      (n!=0 && n%2==1/Inew)
    //         |
    //         v
    //     IntNextBitAllocated(n, shamt)
    //         |
    //       (*/Iinc)
    //         |
    //         v
    //     IntNextBitShifting(n, shamt, i=shamt) ---(i==0/Iadd)---> IntWaitingNextBit(n>>1, shamt=shamt+1)
    //         |
    //       (i!=0/Ishl)---> IntNextBitShifting(n, shamt, i=i-1)
    IntInitial(u64),
    IntWaitingNextBit(u64, usize),
    IntNextBitAllocated(u64, usize),
    IntNextBitShifting(u64, usize, usize),

    // (*) UintInitial(n) ---(*/None)---> UintPushingInt(IntInitial(n)) ---(...)---> UintPushingInt(Done) ---(*/Itou)---> Done
    UintInitial(u64),
    UintPushingInt(Box<State>),

    // (*) FloatInitial(f)
    //         |
    //      (f is NaN/Fnan)---> Done
    //         |
    //      (f is positive infinite/Finf)---> Done
    //         |
    //      (f is negative infinite/Finf)---> FloatWaitingNegation ---(*/Fneg)---> Done
    //         |
    //      (f is finite/None)---> FloatPushingInt(IntInitial(f as bits)) ---(...)---> FloatPushingInt(Done) ---(*/Itof)---> Done
    FloatInitial(f64),
    FloatWaitingNegation,
    FloatPushingInt(Box<State>),

    // (*) StringInitial(s)
    //         |
    //      (*/Snew)
    //         |
    //         v
    //     StringWaitingNextChar(s, len=len(s), i=0) ---(len<=i/None)---> Done
    //         |
    //      (i<len/None)
    //         |
    //         v
    //     StringPushingChar(IntInitial(s[i] as u64), s, len, i)
    //         |
    //       (...)
    //         |
    //         v
    //     StringPushingChar(Done, s, len, i) ---(*/Sadd)---> StringWaitingNextChar(s, len, i=i+1)
    StringInitial(Vec<u8>),
    StringWaitingNextChar(Vec<u8>, usize, usize),
    StringPushingChar(Box<State>, Vec<u8>, usize, usize),

    // (*) ObjectInitial(obj)
    //            |
    //          (*/Onew)
    //            |
    //            v
    //     ObjectWaitingNextItem(it=obj.into_iter()) ---(it.next()==None/None)---> Done
    //            |
    //     (it.next()==Some(k, v)/None)
    //            |
    //            v
    //     ObjectPushingKey(StringInitial(k), v, it) ---(...)---> ObjectPushingKey(Done, v, it)
    //                                                                      |
    //            +-----------------------(*/None)--------------------------+
    //            |
    //            v
    //     ObjectPushingValue(State::new(v), it) ---(...)---> ObjectPushingValue(Done, it)
    //                                                                      |
    //            +-----------------------(*/Oadd)--------------------------+
    //            |
    //            v
    //     ObjectWaitingNextItem(it)
    ObjectInitial(Map),
    ObjectWaitingNextItem(MapIter),
    ObjectPushingKey(Box<State>, Value, MapIter),
    ObjectPushingValue(Box<State>, MapIter),

    // (*) ArrayInitial(arr)
    //           |
    //         (*/Anew)
    //           |
    //           v
    //     ArrayWaitingNextItem(it=arr.into_iter()) ---(it.next()==None/None)---> Done
    //           |
    //         (it.next()==Some(v)/None)
    //           |
    //           v
    //     ArrayPushingElem(State::new(v), it) ---(...)---> ArrayPushingElem(Done, it)
    //                                                                     |
    //           +-----------------------(*/Aadd)--------------------------+
    //           |
    //           v
    //     ArrayWaitingNextItem(it)
    ArrayInitial(Vec<Value>),
    ArrayWaitingNextItem(ArrayIter),
    ArrayPushingElem(Box<State>, ArrayIter),

    // (*) BoolInitial(b)
    //         |
    //       (*/Bnew)
    //         |
    //         v
    //     BoolAllocated(b) ---(b==false/None)---> Done
    //         |
    //       (b==true/Bneg)---> Done
    BoolInitial(bool),
    BoolAllocated(bool),

    // (*) NilInitial ---(*/Nnew)---> Done
    NilInitial,

    // Done ---(*/None)---> Done
    Done,
}

impl State {
    /// Returns a new state.
    fn new(v: Value) -> Self {
        match v {
            Int(i) => State::IntInitial(i as u64),
            Uint(u) => State::UintInitial(u),
            Float(f) => State::FloatInitial(f),
            String(s) => State::StringInitial(s),
            Object(map) => State::ObjectInitial(map),
            Array(arr) => State::ArrayInitial(arr),
            Bool(b) => State::BoolInitial(b),
            Nil => State::NilInitial,
        }
    }

    /// Transitions the state and outputs an instruction.
    fn transition(&mut self) -> Option<Insn> {
        use State::*;

        match self.take() {
            IntInitial(n) => {
                *self = IntWaitingNextBit(n, 0);
                Some(Inew)
            }
            IntWaitingNextBit(n, shamt) => {
                if n == 0 {
                    *self = Done;
                    None
                } else if n % 2 == 0 {
                    *self = IntWaitingNextBit(n >> 1, shamt + 1);
                    None
                } else {
                    *self = IntNextBitAllocated(n, shamt);
                    Some(Inew)
                }
            }
            IntNextBitAllocated(n, shamt) => {
                *self = IntNextBitShifting(n, shamt, shamt);
                Some(Iinc)
            }
            IntNextBitShifting(n, shamt, i) => {
                if i == 0 {
                    *self = IntWaitingNextBit(n >> 1, shamt + 1);
                    Some(Iadd)
                } else {
                    *self = IntNextBitShifting(n, shamt, i - 1);
                    Some(Ishl)
                }
            }
            UintInitial(n) => {
                *self = UintPushingInt(Box::new(IntInitial(n)));
                None
            }
            UintPushingInt(mut boxed_state) => match boxed_state.as_ref() {
                &Done => {
                    *self = Done;
                    Some(Itou)
                }
                _ => {
                    let insn = boxed_state.transition();
                    *self = UintPushingInt(boxed_state);
                    insn
                }
            },
            FloatInitial(f) => {
                if f.is_nan() {
                    *self = Done;
                    Some(Fnan)
                } else if f.is_infinite() {
                    if f.is_sign_negative() {
                        *self = FloatWaitingNegation;
                        Some(Finf)
                    } else {
                        *self = Done;
                        Some(Finf)
                    }
                } else {
                    *self = FloatPushingInt(Box::new(IntInitial(f.to_bits())));
                    None
                }
            }
            FloatWaitingNegation => {
                *self = Done;
                Some(Fneg)
            }
            FloatPushingInt(mut boxed_state) => match boxed_state.as_ref() {
                &Done => {
                    *self = Done;
                    Some(Itof)
                }
                _ => {
                    let insn = boxed_state.transition();
                    *self = FloatPushingInt(boxed_state);
                    insn
                }
            },
            StringInitial(s) => {
                let len = s.len();
                *self = StringWaitingNextChar(s, len, 0);
                Some(Snew)
            }
            StringWaitingNextChar(s, len, i) => {
                if len <= i {
                    *self = Done;
                    None
                } else {
                    *self = StringPushingChar(Box::new(IntInitial(s[i] as u64)), s, len, i);
                    None
                }
            }
            StringPushingChar(mut boxed_state, s, len, i) => match boxed_state.as_ref() {
                &Done => {
                    *self = StringWaitingNextChar(s, len, i + 1);
                    Some(Sadd)
                }
                _ => {
                    let insn = boxed_state.transition();
                    *self = StringPushingChar(boxed_state, s, len, i);
                    insn
                }
            },
            ObjectInitial(map) => {
                *self = ObjectWaitingNextItem(map.into_iter());
                Some(Onew)
            }
            ObjectWaitingNextItem(mut it) => match it.next() {
                None => {
                    *self = Done;
                    None
                }
                Some((k, v)) => {
                    *self = ObjectPushingKey(Box::new(StringInitial(k)), v, it);
                    None
                }
            },
            ObjectPushingKey(mut boxed_state, v, it) => match boxed_state.as_ref() {
                &Done => {
                    *self = ObjectPushingValue(Box::new(State::new(v)), it);
                    None
                }
                _ => {
                    let insn = boxed_state.transition();
                    *self = ObjectPushingKey(boxed_state, v, it);
                    insn
                }
            },
            ObjectPushingValue(mut boxed_state, it) => match boxed_state.as_ref() {
                &Done => {
                    *self = ObjectWaitingNextItem(it);
                    Some(Oadd)
                }
                _ => {
                    let insn = boxed_state.transition();
                    *self = ObjectPushingValue(boxed_state, it);
                    insn
                }
            },
            ArrayInitial(arr) => {
                *self = ArrayWaitingNextItem(arr.into_iter());
                Some(Anew)
            }
            ArrayWaitingNextItem(mut it) => match it.next() {
                None => {
                    *self = Done;
                    None
                }
                Some(v) => {
                    *self = ArrayPushingElem(Box::new(State::new(v)), it);
                    None
                }
            },
            ArrayPushingElem(mut boxed_state, it) => match boxed_state.as_ref() {
                &Done => {
                    *self = ArrayWaitingNextItem(it);
                    Some(Aadd)
                }
                _ => {
                    let insn = boxed_state.transition();
                    *self = ArrayPushingElem(boxed_state, it);
                    insn
                }
            },
            BoolInitial(b) => {
                *self = BoolAllocated(b);
                Some(Bnew)
            }
            BoolAllocated(b) => {
                *self = Done;
                if b {
                    Some(Bneg)
                } else {
                    None
                }
            }
            NilInitial => {
                *self = Done;
                Some(Nnew)
            }
            Done => None,
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
        loop {
            match self.state.transition() {
                None => match self.state {
                    State::Done => {
                        return None;
                    }
                    _ => {
                        continue;
                    }
                },
                Some(insn) => {
                    return Some(insn);
                }
            }
        }
    }

    /// Returns an iterator over the instructions.
    /// TODO: make this IntoIterator
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

    #[test]
    fn serializer_float() {
        assert_eq!(to_insn_vec(Float(f64::NAN)), vec![Fnan]);
        assert_eq!(to_insn_vec(Float(f64::INFINITY)), vec![Finf]);
        assert_eq!(to_insn_vec(Float(f64::NEG_INFINITY)), vec![Finf, Fneg]);

        assert_identical(Float(0.0));
        assert_identical(Float(1.0));
        assert_identical(Float(123.45e-67));
        assert_identical(Float(8.9102e34));
    }

    #[test]
    fn serializer_string() {
        assert_identical(String(Vec::new()));
        assert_identical(String(b"a".to_vec()));
        assert_identical(String(b"ab".to_vec()));
        assert_identical(String(
            b"qawsedrftgyhujikolp;zasxdcfvgbhnjmk,l.;qaswderftgyhujikolp;".to_vec(),
        ));
    }

    #[test]
    fn serializer_object() {
        assert_identical(Object(Map::new()));
        assert_identical(Object(
            vec![(b"key".to_vec(), Int(123))].into_iter().collect(),
        ));
        assert_identical(Object(
            vec![
                (b"key".to_vec(), Int(123)),
                (b"another_key".to_vec(), Float(1.23)),
            ]
            .into_iter()
            .collect(),
        ));
        assert_identical(Object(
            vec![
                (b"key".to_vec(), Int(123)),
                (b"another_key".to_vec(), Float(1.23)),
                (
                    b"nested_object".to_vec(),
                    Object(
                        vec![(b"nested_key".to_vec(), String(b"value".to_vec()))]
                            .into_iter()
                            .collect(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        ));
    }

    #[test]
    fn serializer_array() {
        assert_identical(Array(Vec::new()));
        assert_identical(Array(vec![Int(1)]));
        assert_identical(Array(vec![Int(1), String(b"2".to_vec())]));
        assert_identical(Array(vec![
            Int(1),
            String(b"2".to_vec()),
            Array(vec![Uint(3), String(b"nested".to_vec())]),
        ]));
    }

    #[test]
    fn serializer_bool() {
        assert_identical(Bool(false));
        assert_identical(Bool(true));
    }

    #[test]
    fn serializer_nil() {
        assert_identical(Nil);
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
