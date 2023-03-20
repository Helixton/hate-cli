
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

fn print_an_address() -> Result<()> {
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    let j = serde_json::to_string(&address)?;

    println!("{}", j);

    Ok(())
}

fn main() {
    
    print_an_address();
}