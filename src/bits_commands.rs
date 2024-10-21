pub fn bitboard(binary: Option<String>, decimal: Option<u64>, hex: Option<String>) {
    let num: u64 =
    if let Some(v) = decimal {
       v
    }else if let Some(v)= hex{
        u64::from_str_radix(&v, 16).expect("failed parsing hex string")
    }else if let Some(b) = binary{
        u64::from_str_radix(&b, 2).expect("failed parsing binary string")
    }
    else{
        0
    };

    for rank in (0..8).rev() {
        let byte = ((num >> (rank * 8)) & 0xFF) as u8;
        print!("{:02X} | ", byte);

        for file in (0..8).rev() {
            let bit = (num >> (rank * 8 + file)) & 1;
            print!("{}  ", bit);
        }
        println!();
    }
    
    println!("    -----------------------");
    println!("     7  6  5  4  3  2  1  0");

    println!("dec: {}", num);
    println!("hex: {:X}", num);
    println!("bin: {:b}", num);
    println!("lsb: {}", num.trailing_zeros());
    println!("msb: {}", 63-num.leading_zeros());
    println!("set bits: {}", num.count_ones());
}

pub fn to_binary(decimal: Option<u64>, hex: Option<String>) {
    if let Some(dec) = decimal {
        println!("{:b}", dec);
    } else if let Some(hex_str) = hex {
        if let Ok(decimal_val) = u64::from_str_radix(&hex_str, 16) {
            println!("{:b}", decimal_val);
        } else {
            println!("Invalid hexadecimal input.");
        }
    } else {
        println!("No valid input provided for binary conversion.");
    }
}

pub fn to_hex(decimal: Option<u64>, binary: Option<String>) {
    if let Some(dec) = decimal {
        // Convert decimal to hexadecimal and print
        println!("{:X}", dec);
    } else if let Some(bin) = binary {
        // Convert binary (in decimal form) to hexadecimal and print
        let b = u64::from_str_radix(&bin, 2).expect("failed parsing binary string");
        println!("{:X}", b);
    } else {
        println!("No valid input provided for hexadecimal conversion.");
    }
}

pub fn to_dec(binary: Option<String>, hex: Option<String>) {
    if let Some(bin) = binary {
        // Binary is already in decimal format, just print it
        let b = u64::from_str_radix(&bin, 2).expect("failed parsing binary string");
        println!("{}", b);
    } else if let Some(hex_str) = hex {
        // Convert hex string to decimal
        if let Ok(decimal_val) = u64::from_str_radix(&hex_str, 16) {
            println!("{}", decimal_val);
        } else {
            println!("Invalid hexadecimal input.");
        }
    } else {
        println!("No valid input provided for decimal conversion.");
    }
}
