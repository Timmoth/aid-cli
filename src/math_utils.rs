use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt, recognize},
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::str::FromStr;

// Define the AST for the expressions
#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Variable, // New variant for the variable x
    UnaryOp(Box<Expr>),
    BinaryOp(Box<Expr>, Op, Box<Expr>),
    TrigOp(TrigFunc, Box<Expr>),
    Pi,
    E,
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
        preceded(multispace0, digit1),  // integer part
        opt(preceded(char('.'), digit1)),  // optional fractional part
    )))(input)?;

    let number = f64::from_str(number_str.trim()).unwrap();
    Ok((input, Expr::Number(number)))
}

// A parser for unary operations (e.g., negation)
fn parse_unary(input: &str) -> IResult<&str, Expr> {
    let (input, _) = preceded(multispace0, char('-'))(input)?;
    let (input, expr) = preceded(multispace0, parse_factor)(input)?;
    Ok((input, Expr::UnaryOp(Box::new(expr))))
}

// A parser for pi (π)
fn parse_pi(input: &str) -> IResult<&str, Expr> {
    preceded(multispace0, map(tag("pi"), |_| Expr::Pi))(input)
}

// A parser for e (Euler's number)
fn parse_e(input: &str) -> IResult<&str, Expr> {
    preceded(multispace0, map(tag("e"), |_| Expr::E))(input)
}

// A parser for the variable 'x'
fn parse_variable(input: &str) -> IResult<&str, Expr> {
    preceded(multispace0, map(tag("x"), |_| Expr::Variable))(input)
}

// A parser for trigonometric functions
fn parse_trig_func(input: &str) -> IResult<&str, Expr> {
    let (input, trig_func) = preceded(multispace0,alt((
        map(tag("sin"), |_| TrigFunc::Sin),
        map(tag("cos"), |_| TrigFunc::Cos),
        map(tag("tan"), |_| TrigFunc::Tan),
    )))(input)?;

    let (input, expr) = preceded(multispace0, delimited(char('('), parse_expr, char(')')))(input)?;
    Ok((input, Expr::TrigOp(trig_func, Box::new(expr))))
}

// A parser for multiplication, division, and other operators
fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, mut left) = parse_factor(input)?; // Start with parsing a factor
    let mut input = input;

    while let Ok((next_input, op)) = parse_mul_div_op(input) {
        let (next_input, right) = parse_factor(next_input)?; // Continue parsing factors for terms
        left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        input = next_input;
    }

    Ok((input, left))
}
// A parser for basic factors: numbers, parentheses, 'x', etc.
fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((
        delimited(
            preceded(multispace0, char('(')),
            parse_expr,
            preceded(multispace0, char(')')),
        ),
        parse_number,
        parse_variable,
        parse_unary,
        parse_pi,
        parse_e,
        parse_trig_func, // Trigonometric functions should stay in factors
    ))(input)
}

// A parser for addition and subtraction
fn parse_op(input: &str) -> IResult<&str, Op> {
    preceded(multispace0, alt((
        map(tag("+"), |_| Op::Add),
        map(tag("-"), |_| Op::Sub),
    )))(input)
}

// A parser for multiplication, division, and exponentiation
fn parse_mul_div_op(input: &str) -> IResult<&str, Op> {
    preceded(multispace0, alt((
        map(tag("*"), |_| Op::Mul),
        map(tag("/"), |_| Op::Div),
        map(tag("^"), |_| Op::Pow),
        map(tag("%"), |_| Op::Mod),
    )))(input)
}

// A parser for binary operations (addition, subtraction, etc.)
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
pub fn evaluate(expr: &Expr, x_value: f64) -> f64 {
    match expr {
        Expr::Number(value) => *value,
        Expr::Variable => x_value,  // Substitute variable 'x' with its value
        Expr::Pi => std::f64::consts::PI,
        Expr::E => std::f64::consts::E,
        Expr::UnaryOp(inner) => -evaluate(inner, x_value),
        Expr::BinaryOp(left, op, right) => {
            let left_value = evaluate(left, x_value);
            let right_value = evaluate(right, x_value);
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
            let arg_value = evaluate(arg, x_value);
            match func {
                TrigFunc::Sin => arg_value.sin(),
                TrigFunc::Cos => arg_value.cos(),
                TrigFunc::Tan => arg_value.tan(),
            }
        }
    }
}