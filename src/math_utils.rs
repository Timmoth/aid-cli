use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::{map, map_res, opt, recognize},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use core::f64;
use std::str::FromStr;

// Define the AST for the expressions
#[derive(Debug)]
pub enum Expr {
    Number(f64),
    UnaryOp(Box<Expr>),
    BinaryOp(Box<Expr>, Op, Box<Expr>),
    TrigOp(TrigFunc, Box<Expr>),
    Pi,
    E
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
}

#[derive(Debug)]
pub enum TrigFunc {
    Sin,
    Cos,
    Tan,
}

// A parser for floating-point numbers
fn parse_number(input: &str) -> IResult<&str, Expr> {
    let (input, number_str) = recognize(tuple((
                preceded(multispace0, digit1), // integer part
                opt(preceded(char('.'), digit1)), // optional fractional part
            )))(input)?;

    // Convert the recognized number string to a floating-point number
    let number = f64::from_str(number_str.trim()).unwrap(); // safely unwrap for valid input
    Ok((input, Expr::Number(number)))
}

// A parser for unary operations (e.g., negation)
fn parse_unary(input: &str) -> IResult<&str, Expr> {
    let (input, _) = preceded(multispace0, char('-'))(input)?;
    let (input, expr) = preceded(multispace0, parse_factor)(input)?;
    Ok((input, Expr::UnaryOp(Box::new(expr))))
}

fn parse_pi(input: &str) -> IResult<&str, Expr> {
    preceded(multispace0, map(tag("pi"), |_| Expr::Pi))(input)
}
fn parse_e(input: &str) -> IResult<&str, Expr> {
    preceded(multispace0, map(tag("e"), |_| Expr::E))(input)
}
// A parser for multiplication and division
fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, mut left) = parse_factor(input)?;

    let mut input = input;

    while let Ok((next_input, op)) = parse_mul_div_op(input) {
        let (next_input, right) = parse_factor(next_input)?;
        left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        input = next_input;
    }

    Ok((input, left))
}

// A parser for the basic factors: numbers and parentheses
fn parse_factor(input: &str) -> IResult<&str, Expr> {
    let (input, result) = alt((
        delimited(
            preceded(multispace0, char('(')),
            parse_expr,
            preceded(multispace0, char(')')),
        ),
        parse_number,
        parse_unary,
        parse_trig_func,
        parse_pi,
        parse_e
    ))(input)?;

    Ok((input, result))
}

// A parser for addition and subtraction
fn parse_op(input: &str) -> IResult<&str, Op> {
    preceded(multispace0, alt((
        map(tag("+"), |_| Op::Add),
        map(tag("-"), |_| Op::Sub),
    )))(input)
}

// A parser for multiplication and division
fn parse_mul_div_op(input: &str) -> IResult<&str, Op> {
    preceded(multispace0, alt((
        map(tag("*"), |_| Op::Mul),
        map(tag("/"), |_| Op::Div),
        map(tag("^"), |_| Op::Pow),
        map(tag("%"), |_| Op::Mod),
    )))(input)
}

// A parser for trigonometric functions
fn parse_trig_func(input: &str) -> IResult<&str, Expr> {
    let (input, trig_func) = alt((
        map(tag("sin"), |_| TrigFunc::Sin),
        map(tag("cos"), |_| TrigFunc::Cos),
        map(tag("tan"), |_| TrigFunc::Tan),
    ))(input)?;
    
    let (input, expr) = preceded(multispace0, delimited(char('('), parse_expr, char(')')))(input)?;
    Ok((input, Expr::TrigOp(trig_func, Box::new(expr))))
}

// A parser for binary operations
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let (input, mut left) = parse_term(input)?;

    let mut input = input;

    while let Ok((next_input, op)) = parse_op(input) {
        let (next_input, right) = parse_term(next_input)?;
        left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        input = next_input;
    }

    Ok((input, left))
}

// Main evaluation function
pub fn evaluate(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(value) => *value,
        Expr::Pi => std::f64::consts::PI,
        Expr::E => std::f64::consts::E,
        Expr::UnaryOp(inner) => -evaluate(inner),
        Expr::BinaryOp(left, op, right) => {
            let left_value = evaluate(left);
            let right_value = evaluate(right);
            match op {
                Op::Add => left_value + right_value,
                Op::Sub => left_value - right_value,
                Op::Mul => left_value * right_value,
                Op::Div => left_value / right_value,
                Op::Pow => left_value.powf(right_value),
                Op::Mod => left_value % right_value,
            }
        },
        Expr::TrigOp(func, arg) => {
            let arg_value = evaluate(arg);
            match func {
                TrigFunc::Sin => arg_value.sin(),
                TrigFunc::Cos => arg_value.cos(),
                TrigFunc::Tan => arg_value.tan(),
            }
        }
    }
}