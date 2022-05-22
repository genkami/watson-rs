use watson::vm::VM;
use watson::{Insn, Location, Token};

fn main() -> watson::Result<()> {
    let mut vm = VM::new();

    // Instructions are defined in https://github.com/genkami/watson/blob/main/doc/spec.md.
    vm.execute(token(Insn::Inew))?; // Push a signed integer `0` to the stack.
    vm.execute(token(Insn::Iinc))?; // Increment a value on the top of the stack.
    vm.execute(token(Insn::Ishl))?; // Shift a value on the top of the stack to the left by one bit.

    println!("result: {:?}", vm.peek_top().unwrap());
    Ok(())
}

fn token(insn: Insn) -> Token {
    Token {
        insn: insn,
        location: Location::unknown(),
    }
}
