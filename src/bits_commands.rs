use crate::bits_utils;

pub fn evaluate(board: bool, chess_board: bool, binary: bool, hex: bool, expression: String) {
    let (_, parsed_expr) = bits_utils::parse_expr(&expression).unwrap();
    let result = parsed_expr.eval();
    if board || chess_board {
        bitboard(chess_board, None, Some(result), None)
    } else {
        if binary {
            println!("{:b}", result);
        } else if hex {
            println!("{:x}", result);
        } else {
            println!("{}", result);
        }
    }
}

fn bitboard(chess_board: bool, binary: Option<String>, decimal: Option<u64>, hex: Option<String>) {
    let num: u64 = if let Some(v) = decimal {
        v
    } else if let Some(v) = hex {
        u64::from_str_radix(&v, 16).expect("failed parsing hex string")
    } else if let Some(b) = binary {
        u64::from_str_radix(&b, 2).expect("failed parsing binary string")
    } else {
        0
    };

    for rank in (0..8).rev() {
        let byte = ((num >> (rank * 8)) & 0xFF) as u8;
        if chess_board{
            print!("{} | ", rank + 1);
        }else{
            print!("{:02X} | ", byte);
        }

        if chess_board {
            for file in 0..8 {
                let bit = (num >> (rank * 8 + file)) & 1;
                print!("{}  ", bit);
            }
        } else {
            for file in (0..8).rev() {
                let bit = (num >> (rank * 8 + file)) & 1;
                print!("{}  ", bit);
            }
        }

        println!();
    }

    if chess_board {
        println!("   -----------------------");
        println!("    A  B  C  D  E  F  G  H");
    }else{
        println!("    -----------------------");
        println!("     7  6  5  4  3  2  1  0");
    }

    println!("dec: {}", num);
    println!("hex: {:X}", num);
    println!("bin: {:b}", num);
    println!("lsb: {}", num.trailing_zeros());
    if num.leading_zeros() >= 64 {
        println!("msb: {}", 64);
    } else {
        println!("msb: {}", 63 - num.leading_zeros());
    }
    println!("set bits: {}", num.count_ones());
}