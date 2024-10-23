use std::io::{self, Read};

pub fn args_or_readline(args: Vec<String>) -> String{
     if args.is_empty() {
            // If no arguments are provided, read from stdin
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read from stdin");
            input.trim().to_string() // Trim any whitespace from stdin input
        } else {
            // Concatenate provided arguments with spaces
            args.join(" ")
        }
}