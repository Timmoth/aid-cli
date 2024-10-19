use std::io::{self, Read};
use serde_json::Value;

pub async fn json_extract(property: String) {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
        let json: Value = serde_json::from_str(&input).expect("Failed to parse JSON");

        let filter: &str = &format!("/{}", property);
        match json.pointer(filter) {
            Some(value) => println!("{}", value),
            None => eprintln!("Field not found in the provided JSON"),
        }
}