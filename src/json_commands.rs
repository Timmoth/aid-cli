use std::io::{self, Read};
use serde_derive::Serialize;
use serde_json::{from_str, Value};
use base64::prelude::*;
use std::str;
use base64::engine::general_purpose::URL_SAFE;

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

#[derive(Serialize)]
struct JwtDecoded {
    header: Value,
    payload: Value,
}

pub fn json_decode_jwt(token: &str) {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        println!("Invalid token format");
        return;
    }

    let header = parts[0];
    let payload = parts[1];

    // Decode the header
    let decoded_header = decode_jwt_part(header).unwrap_or_else(|err| {
        println!("Error decoding header: {}", err);
        "".to_string()
    });

    // Decode the payload
    let decoded_payload = decode_jwt_part(payload).unwrap_or_else(|err| {
        println!("Error decoding payload: {}", err);
        "".to_string()
    });

    // Create a struct to hold the decoded values
    let decoded_jwt = JwtDecoded {
        header: from_str(&decoded_header).unwrap_or(Value::Null),
        payload: from_str(&decoded_payload).unwrap_or(Value::Null),
    };

    // Serialize the struct to JSON
    let json_output = serde_json::to_string_pretty(&decoded_jwt).unwrap();

    // Print the JSON output
    println!("{}", json_output);
}


fn decode_jwt_part(part: &str) -> Result<String, String> {
    // Ensure correct padding for Base64 URL encoding
    let mut padding = String::new();
    let missing_padding = 4 - (part.len() % 4);
    if missing_padding != 4 {
        padding.push_str(&"=".repeat(missing_padding));
    }

    // Create the Base64 URL string with proper padding
    let base64_url_encoded = format!("{}{}", part, padding);

    // Decode the Base64 string
    match URL_SAFE.decode(&base64_url_encoded) {
        Ok(decoded_bytes) => {
            // Convert bytes to string
            str::from_utf8(&decoded_bytes)
                .map(|s| s.to_string())
                .map_err(|e| e.to_string())
        }
        Err(e) => Err(format!("Base64 decode error: {}", e)),
    }
}