# watson_rs
[![Test](https://github.com/genkami/watson-rs/actions/workflows/test.yaml/badge.svg)](https://github.com/genkami/watson-rs/actions/workflows/test.yaml)

A Rust implementation of [Wasted but Amazing Turing-incomplete Stack-based Object Notation (WATSON)](https://github.com/genkami/watson).

## Documentation
* [Language specification](https://github.com/genkami/watson/blob/main/doc/spec.md)
* [Original implementation](https://github.com/genkami/watson)

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
watson_rs = "0.1.0"
```

## Examples

### Basic Usage (with serde_watson crate)

```rust
use std::io;

use serde::Serialize;
use serde_watson::ser::Serializer;

#[derive(Serialize)]
struct Transaction {
    id: u64,
    from: String,
    to: String,
    amount: f64,
}

fn main() -> serde_watson::Result<()> {
    let tx = Transaction {
        id: 0xabcd1234,
        from: "Motoaki Tanigo".to_owned(),
        to: "Oozora Subaru".to_owned(),
        amount: 123.45,
    };
    let mut ser = Serializer::from_writer(io::stdout());
    tx.serialize(&mut ser)?;
    Ok(())
}
```

You can run the example above by running `cargo run --example basic` in the [watson_example](https://github.com/genkami/watson-rs/tree/main/watson_examples) directory.

```
$ cargo run --example basic
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/examples/basic`
~?SShkShaaakShaaaaakShaaaaaak-SShaakShaaaaakShaaaaaak-SShaakShaaaakShaaaaakShaaa
aaaaaakShaaaaaaaaaaaakShaaaaaaaaaaaaaaaakShaaaaaaaaaaaaaaaaaakShaaaaaaaaaaaaaaaa
aaakShaaaaaaaaaaaaaaaaaaaaaakShaaaaaaaaaaaaaaaaaaaaaaakShaaaaaaaaaaaaaaaaaaaaaaa
akShaaaaaaaaaaaaaaaaaaaaaaaaakShaaaaaaaaaaaaaaaaaaaaaaaaaaakShaaaaaaaaaaaaaaaaaa
aaaaaaaaaaakShaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaakig$BBubaBubbaBubbbbbaBubbbbbba!BBu
baBubbbbaBubbbbbaBubbbbbba!BBuaBubaBubbaBubbbaBubbbbbaBubbbbbba!BBuaBubbaBubbbaB
ubbbbbaBubbbbbba!?SShkShaakShaaakShaaaaaak-SShkShakShaakShaaakShaaaaakShaaaaaak-
SShaakShaaaakShaaaaakShaaaaaak-SShkShakShaakShaaakShaaaaakShaaaaaak-SShkShaaaaak
Shaaaaaak-SShkShakShaaakShaaaaakShaaaaaak-SShkShaaakShaaaaakShaaaaaak-SShaaaaak-
SShaakShaaaakShaaaaaak-SShkShaaaaakShaaaaaak-SShakShaakShaaakShaaaaakShaaaaaak-S
ShkShaaakShaaaaakShaaaaaak-SShkShakShaakShaaaaakShaaaaaak-SShkShakShaakShaaakSha
aaaakShaaaaaak-g$BBubbaBubbbbaBubbbbbaBubbbbbba!BBuaBubaBubbaBubbbaBubbbbbaBubbb
bbba!?SShkShakShaakShaaakShaaaaaak-SShkShakShaakShaaakShaaaaakShaaaaaak-SShakSha
aakShaaaakShaaaaakShaaaaaak-SShkShakShaakShaaakShaaaaakShaaaaaak-SShakShaaaakSha
aaaakShaaaaaak-SShkShaaaaakShaaaaaak-SShaaaaak-SShkShakShaaaakShaaaaaak-SShkShaa
kShaaaakShaaaaakShaaaaaak-SShakShaaaaakShaaaaaak-SShkShaaaaakShaaaaaak-SShakShaa
aakShaaaaakShaaaaaak-SShkShaakShaaaakShaaaaakShaaaaaak-g$BBuaBubbbbbaBubbbbbba!B
BuaBubbaBubbbaBubbbbbaBubbbbbba!BBuaBubaBubbaBubbbaBubbbbbaBubbbbbba!BBuaBubbaBu
bbbbaBubbbbbaBubbbbbba!BBubaBubbaBubbbaBubbbbbaBubbbbbba!BBubbaBubbbbaBubbbbbaBu
bbbbbba!BBuaBubbaBubbbaBubbbbbbaBubbbbbbbaBubbbbbbbbbbaBubbbbbbbbbbbaBubbbbbbbbb
bbbbbaBubbbbbbbbbbbbbbbaBubbbbbbbbbbbbbbbbbbaBubbbbbbbbbbbbbbbbbbbaBubbbbbbbbbbb
bbbbbbbbbbbaBubbbbbbbbbbbbbbbbbbbbbbbaBubbbbbbbbbbbbbbbbbbbbbbbbbbaBubbbbbbbbbbb
bbbbbbbbbbbbbbbbaBubbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaBubbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbaBubbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaBubbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaB
ubbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaBubbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
baBubbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaBubbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbaBubbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaBubbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbaBubbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaB
ubbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaBubbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbaBubbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaBu
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbaBubbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbbbbbbbbbbbbbbbbaBubbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
bbbbbbbbbbbbbbaiM
```

You can verify the output using the [WATSON CLI](https://github.com/genkami/watson/blob/main/doc/cli.md).

```
$ cargo run --example basic | watson decode -t json
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/examples/basic`
{"amount":123.45,"from":"Motoaki Tanigo","id":2882343476,"to":"Oozora Subaru"}
```

### Run a WATSON VM directly

You can run a WATSON Virtual Machine directly by using this crate:

```rust
use watson_rs::vm::VM;
use watson_rs::{Insn, Location, Token};

fn main() -> watson_rs::Result<()> {
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
```


You can run the example above by running `cargo run --example run_vm` in the [watson_example](https://github.com/genkami/watson-rs/tree/main/watson_examples) directory.

```
$ cargo run --example run_vm
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/run_vm`
result: Int(2)
```