use base64::prelude::*;
use std::str;
use base64::engine::general_purpose::URL_SAFE;

pub fn base64_encode(input: String) {
   // Encode the input string as bytes into Base64 URL format
    let encoded = URL_SAFE.encode(input.as_bytes());
    // Print the Base64 encoded string
    println!("{}", encoded);
}

pub fn base64_decode(input: String) {
// Ensure correct padding for Base64 URL encoding
    let mut padding = String::new();
    let missing_padding = 4 - (input.len() % 4);
    if missing_padding != 4 {
        padding.push_str(&"=".repeat(missing_padding));
    }

    // Create the Base64 URL string with proper padding
    let base64_encoded = format!("{}{}", input, padding);

    // Decode the Base64 string
    match URL_SAFE.decode(&base64_encoded) {
        Ok(decoded_bytes) => {
            // Convert bytes to string
            let decoded_string = str::from_utf8(&decoded_bytes)
                .map(|s| s.to_string())
                .map_err(|e| e.to_string());

            if let Ok(decoded_string) = decoded_string{
                println!("{}", decoded_string);
            }else if let Err(e) = decoded_string{
                eprint!("{}", e);
            }
        }
        Err(e) => eprintln!("Base64 decode error: {}", e),
    }
}
