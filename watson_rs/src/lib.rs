use std::str::FromStr;

pub mod error;
pub mod language;
pub mod lexer;
pub mod serializer;
pub mod unlexer;
pub mod vm;

pub use error::{Error, ErrorKind, Result};
pub use language::{Bytes, Insn, IsValue, Location, Map, ToBytes, Token, Value};
pub use vm::VM;

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Value> {
        let mut bytes = s.as_bytes();
        let mut vm = vm::VM::new();
        vm.execute_all(lexer::Lexer::new(&mut bytes))?;
        vm.into_top().map(Ok).unwrap_or_else(|| {
            Err(Error {
                kind: ErrorKind::EmptyStack,
                location: Location::unknown(),
                source: None,
            })
        })
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    use Value::*;

    #[test]
    fn parse_watson() -> Result<()> {
        assert_eq!("B".parse::<Value>()?, Int(0));
        assert_eq!("BBubba".parse::<Value>()?, Int(4));
        assert_eq!("?SShaaarrk".parse::<Value>()?, Int(8));
        Ok(())
    }
}
