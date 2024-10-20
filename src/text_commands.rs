use base64::prelude::*;
use regex::Regex;
use std::{fs::File, io::{self, BufRead}, str};
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

pub fn regex_search(filename: String, regex: String){
    let file = File::open(filename);
    if let Ok(file) = file{
        let reader = io::BufReader::new(file);
        let re: Regex = Regex::new(regex.as_str()).unwrap();

        for (index, line) in reader.lines().enumerate() {
            if let Ok(line) = line{
                if re.is_match(&line) {
                    println!("{}: {}", index + 1, line);
                }
            }
        }
    }
}

pub fn print_lines(
    filename: String, 
    start: Option<usize>, 
    end: Option<usize>, 
    head: Option<usize>, 
    tail: Option<usize>
) {
    // Check for conflicting options
    if (head.is_some() || tail.is_some()) && (start.is_some() || end.is_some()) {
        eprintln!("Error: Cannot specify both head/tail and start/end ranges.");
        return;
    }
    if head.is_some() && tail.is_some() {
        eprintln!("Error: Cannot specify both head and tail.");
        return;
    }

    // Open the file
    let file = File::open(&filename).expect("Unable to open file");
    let reader = io::BufReader::new(file);
    
    // Handle head: print first N lines
    if let Some(n) = head {
        for (i, line) in reader.lines().enumerate() {
            if i >= n {
                break;
            }
            if let Ok(line) = line {
                println!("{}", line);
            }
        }
        return;
    }

    // If tail is specified, we need to scan the file and count lines first
    if let Some(n) = tail {
        let mut lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                lines.push(line);
            }
        }
        let total_lines = lines.len();
        let start_tail_idx = total_lines.saturating_sub(n);
        for line in lines.iter().skip(start_tail_idx) {
            println!("{}", line);
        }
        return;
    }

    // Handle start/end range without loading the entire file into memory
    let start_idx = start.unwrap_or(0); // If start is None, default to the beginning
    let end_idx = end.unwrap_or(usize::MAX); // If end is None, default to the max usize

    for (index, line) in reader.lines().enumerate() {
        if index < start_idx {
            continue;
        }
        if index > end_idx {
            break;
        }
        if let Ok(line) = line {
            println!("{}", line);
        }
    }
}
