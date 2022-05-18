use crate::error::Result;
use crate::language::{Bytes, Insn, Map, Value};
use Insn::*;
use Value::*;

/// A trait for objects that can be used as a sink of instructions.
pub trait WriteInsn {
    /// Writes a single instruction.
    fn write(&mut self, insn: Insn) -> Result<()>;

    /// Writes all instructions.
    fn write_all(&mut self, insns: &[Insn]) -> Result<()> {
        for i in insns {
            self.write(*i)?;
        }
        Ok(())
    }
}

impl<'a> WriteInsn for &'a mut Vec<Insn> {
    fn write(&mut self, insn: Insn) -> Result<()> {
        self.push(insn);
        Ok(())
    }
}

/// Serializer converts `Value` into a sequence of `Insn`s.
pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W> {
    /// Returns a new `Serializer`.
    pub fn new(writer: W) -> Self {
        Serializer { writer: writer }
    }
}

impl<W: WriteInsn> Serializer<W> {
    /// Serializes a single `Value`.
    pub fn serialize(&mut self, v: &Value) -> Result<()> {
        match v {
            &Int(n) => self.serialize_int(n),
            &Uint(n) => self.serialize_uint(n),
            &Float(f) => self.serialize_float(f),
            &String(ref s) => self.serialize_string(s),
            &Object(ref map) => self.serialize_object(map),
            &Array(ref arr) => self.serialize_array(arr),
            &Bool(b) => self.serialize_bool(b),
            &Nil => self.serialize_nil(),
        }
    }

    fn serialize_int(&mut self, n: i64) -> Result<()> {
        let mut n = n as u64;
        self.write(Inew)?;
        let mut shift: usize = 0;
        while n != 0 {
            if n % 2 == 1 {
                self.write_all(&[Inew, Iinc])?;
                for _ in 1..=shift {
                    self.write(Ishl)?;
                }
                self.write(Iadd)?;
            }
            n = n >> 1;
            shift += 1;
        }
        Ok(())
    }

    fn serialize_uint(&mut self, n: u64) -> Result<()> {
        self.serialize_int(n as i64)?;
        self.write(Itou)
    }

    fn serialize_float(&mut self, f: f64) -> Result<()> {
        if f.is_nan() {
            self.write(Fnan)
        } else if f.is_infinite() {
            self.write(Finf)?;
            if f.is_sign_negative() {
                self.write(Fneg)?;
            }
            Ok(())
        } else {
            self.serialize_int(f.to_bits() as i64)?;
            self.write(Itof)
        }
    }

    fn serialize_string(&mut self, s: &Bytes) -> Result<()> {
        self.write(Snew)?;
        for c in s {
            self.serialize_int(*c as i64)?;
            self.write(Sadd)?;
        }
        Ok(())
    }

    fn serialize_object(&mut self, map: &Map) -> Result<()> {
        self.write(Onew)?;
        for (k, v) in map {
            self.serialize_string(k)?;
            self.serialize(v)?;
            self.write(Oadd)?;
        }
        Ok(())
    }

    fn serialize_array(&mut self, arr: &Vec<Value>) -> Result<()> {
        self.write(Anew)?;
        for i in arr {
            self.serialize(i)?;
            self.write(Aadd)?;
        }
        Ok(())
    }

    fn serialize_bool(&mut self, b: bool) -> Result<()> {
        self.write(Bnew)?;
        if b {
            self.write(Bneg)?;
        }
        Ok(())
    }

    fn serialize_nil(&mut self) -> Result<()> {
        self.write(Nnew)
    }

    fn write(&mut self, i: Insn) -> Result<()> {
        self.writer.write(i)
    }

    fn write_all(&mut self, insns: &[Insn]) -> Result<()> {
        self.writer.write_all(insns)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::language::{Location, Token};
    use crate::vm;

    #[test]
    fn serializer_int() {
        assert_eq!(to_insn_vec(&Int(0)), vec![Inew]);
        assert_eq!(to_insn_vec(&Int(1)), vec![Inew, Inew, Iinc, Iadd]);
        assert_eq!(to_insn_vec(&Int(2)), vec![Inew, Inew, Iinc, Ishl, Iadd]);
        assert_eq!(
            to_insn_vec(&Int(3)),
            vec![Inew, Inew, Iinc, Iadd, Inew, Iinc, Ishl, Iadd]
        );
        assert_eq!(
            to_insn_vec(&Int(0b1010101)),
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
        assert_eq!(to_insn_vec(&Float(f64::NAN)), vec![Fnan]);
        assert_eq!(to_insn_vec(&Float(f64::INFINITY)), vec![Finf]);
        assert_eq!(to_insn_vec(&Float(f64::NEG_INFINITY)), vec![Finf, Fneg]);

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

    fn to_insn_vec(value: &Value) -> Vec<Insn> {
        let mut insns = Vec::new();
        Serializer::new(&mut insns).serialize(&value).unwrap();
        insns
    }

    fn assert_identical(value: Value) {
        let mut vm = vm::VM::new();
        for insn in to_insn_vec(&value) {
            vm.execute(Token {
                insn: insn,
                location: Location::unknown(),
            })
            .expect("execution error");
        }
        let result = vm.peek_top().expect("stack is empty");
        assert_eq!(&value, result);
    }
}
