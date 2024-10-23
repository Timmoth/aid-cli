use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, space0},
    combinator::{map, map_res},
    sequence::{delimited, preceded},
    IResult,
};
use std::str::FromStr;

#[derive(Debug)]
pub enum Expr {
    Num(u64),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
}

#[derive(Debug)]
pub enum BinOp {
    And,
    Or,
    Xor,
    Shl,
    Shr,
}

#[derive(Debug)]
pub enum UnOp {
    Not,
}

// Parsing numbers: decimal, hex, and binary
fn parse_number(input: &str) -> IResult<&str, u64> {
    alt((
        map_res(preceded(tag("0x"), nom::character::complete::hex_digit1), |s: &str| {
            u64::from_str_radix(s, 16)
        }),
        map_res(preceded(tag("0b"), nom::character::complete::digit1), |s: &str| {
            u64::from_str_radix(s, 2)
        }),
        map_res(digit1, |s: &str| u64::from_str(s)),
    ))(input)
}

// Parsing parenthesized expressions
fn parse_parentheses(input: &str) -> IResult<&str, Expr> {
    delimited(
        delimited(space0, char('('), space0),
        |input| parse_expression(input), // Call the main expression parser
        delimited(space0, char(')'), space0),
    )(input)
}

// Primary expressions: either numbers or parentheses
fn parse_primary(input: &str) -> IResult<&str, Expr> {
    alt((parse_parentheses, map(parse_number, Expr::Num)))(input)
}

// Parsing unary operator (NOT)
fn parse_unop(input: &str) -> IResult<&str, UnOp> {
    delimited(space0, map(tag("!"), |_| UnOp::Not), space0)(input)
}

// Function to parse unary expressions
fn parse_unary_expr(input: &str) -> IResult<&str, Expr> {
    let (input, op) = parse_unop(input)?;
    let (input, expr) = parse_primary(input)?;
    Ok((input, Expr::UnOp(op, Box::new(expr))))
}

// Binary operator precedence levels
fn get_precedence(op: &BinOp) -> u8 {
    match op {
        BinOp::Shl | BinOp::Shr => 3,
        BinOp::And => 2,
        BinOp::Xor => 1,
        BinOp::Or => 0,
    }
}

// Parsing binary operators
fn parse_binop(input: &str) -> IResult<&str, BinOp> {
    alt((
        map(tag("&"), |_| BinOp::And),
        map(tag("|"), |_| BinOp::Or),
        map(tag("^"), |_| BinOp::Xor),
        map(tag("<<"), |_| BinOp::Shl),
        map(tag(">>"), |_| BinOp::Shr),
    ))(input)
}

// Parse expressions with precedence climbing
fn parse_expression(input: &str) -> IResult<&str, Expr> {
    let (input, mut lhs) = alt((parse_unary_expr, parse_primary))(input)?; // Allow unary expressions first

    let mut input = input;
    loop {
        let (next_input, _) = space0(input)?;

        // Attempt to parse a binary operator
        if let Ok((next_input, op)) = parse_binop(next_input) {
            let precedence = get_precedence(&op);
            let (next_input, _) = space0(next_input)?;
            let (next_input, mut rhs) = alt((parse_unary_expr, parse_primary))(next_input)?; // Allow unary expressions on rhs

            // Check for next higher precedence operators
            while let Ok((next_next_input, next_op)) = parse_binop(next_input) {
                let next_precedence = get_precedence(&next_op);
                if next_precedence > precedence {
                    let (next_next_input, higher_rhs) = parse_expression(next_next_input)?;
                    rhs = Expr::BinOp(Box::new(rhs), next_op, Box::new(higher_rhs));
                    input = next_next_input; // Update input
                } else {
                    input = next_next_input; // Exit if no higher precedence
                    break;
                }
            }

            lhs = Expr::BinOp(Box::new(lhs), op, Box::new(rhs));
            input = next_input; // Update input for the next iteration
        } else {
            break; // Exit loop if no binary operator found
        }
    }

    Ok((input, lhs))
}

// Parsing the expression (with support for unary and binary operators)
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    parse_expression(input)
}

// Implement evaluation of the expressions
impl Expr {
    pub fn eval(&self) -> u64 {
        match self {
            Expr::Num(n) => *n,
            Expr::BinOp(lhs, op, rhs) => {
                let lhs_val = lhs.eval();
                let rhs_val = rhs.eval();
                match op {
                    BinOp::And => lhs_val & rhs_val,
                    BinOp::Or => lhs_val | rhs_val,
                    BinOp::Xor => lhs_val ^ rhs_val,
                    BinOp::Shl => lhs_val << rhs_val,
                    BinOp::Shr => lhs_val >> rhs_val,
                }
            }
            Expr::UnOp(op, expr) => {
                let val = expr.eval();
                match op {
                    UnOp::Not => !val,
                }
            }
        }
    }
}