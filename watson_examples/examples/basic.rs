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
