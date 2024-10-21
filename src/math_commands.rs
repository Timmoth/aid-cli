use crate::math_utils;

pub fn evaluate(expression: String) {
    match math_utils::parse_expr(&expression) {
        Ok((_, ast)) => {
            println!("{}", math_utils::evaluate(&ast));
        }
        Err(e) => println!("Error parsing expression: {:?}", e),
    }
}
