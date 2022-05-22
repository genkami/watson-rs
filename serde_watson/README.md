# serde_watson

Serde support for esoteric configuration language [WATSON](https://github.com/genkami/watson).

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
serde_watson = "0.1.0"
```

## Examples

### Basic Usage

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