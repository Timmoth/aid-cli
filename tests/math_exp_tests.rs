use aid::math_utils;

#[cfg(test)]
mod tests {
    use core::f64;

    use super::*;

    #[test]
    fn addition() {
        let expression = "1 + 2";
        let parse_result = math_utils::parse_expr(expression);
        if let Err(e) = &parse_result {
            eprint!("{}", e);
            assert!(!parse_result.is_err());
        }
        
        let (_, ast) = parse_result.unwrap();
        let eval = math_utils::evaluate(&ast, 0.0);
        assert_eq!(3.0, eval);
    }

    #[test]
    fn subtraction() {
        let expression = "1 - 2";
        let parse_result = math_utils::parse_expr(expression);
        if let Err(e) = &parse_result {
            eprint!("{}", e);
            assert!(!parse_result.is_err());
        }
        
        let (_, ast) = parse_result.unwrap();
        let eval = math_utils::evaluate(&ast, 0.0);
        assert_eq!(-1.0, eval);
    }

    #[test]
    fn multiplication() {
        let expression = "1 * 2";
        let parse_result = math_utils::parse_expr(expression);
        if let Err(e) = &parse_result {
            eprint!("{}", e);
            assert!(!parse_result.is_err());
        }
        
        let (_, ast) = parse_result.unwrap();
        let eval = math_utils::evaluate(&ast, 0.0);
        assert_eq!(2.0, eval);
    }

    #[test]
    fn division() {
        let expression = "1 / 2";
        let parse_result = math_utils::parse_expr(expression);
        if let Err(e) = &parse_result {
            eprint!("{}", e);
            assert!(!parse_result.is_err());
        }
        
        let (_, ast) = parse_result.unwrap();
        let eval = math_utils::evaluate(&ast, 0.0);
        assert_eq!(0.5, eval);
    }

    #[test]
    fn exponent() {
        let expression = "2 ^ 3";
        let parse_result = math_utils::parse_expr(expression);
        if let Err(e) = &parse_result {
            eprint!("{}", e);
            assert!(!parse_result.is_err());
        }
        
        let (_, ast) = parse_result.unwrap();
        let eval = math_utils::evaluate(&ast, 0.0);
        assert_eq!(8.0, eval);
    }

    #[test]
    fn sin() {
        let expression = "sin(pi)";
        let parse_result = math_utils::parse_expr(expression);
        if let Err(e) = &parse_result {
            eprint!("{}", e);
            assert!(!parse_result.is_err());
        }
        
        let (_, ast) = parse_result.unwrap();
        let eval = math_utils::evaluate(&ast, 0.0);
        let expected = 0.0;
        assert!((eval - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn tan() {
        let expression = "tan(pi/4)";
        let parse_result = math_utils::parse_expr(expression);
        if let Err(e) = &parse_result {
            eprint!("{}", e);
            assert!(!parse_result.is_err());
        }
        
        let (_, ast) = parse_result.unwrap();
        let eval = math_utils::evaluate(&ast, 0.0);
        let expected = 1.0;
        assert!((eval - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn cos() {
        let expression = "cos(pi)";
        let parse_result = math_utils::parse_expr(expression);
        if let Err(e) = &parse_result {
            eprint!("{}", e);
            assert!(!parse_result.is_err());
        }
        
        let (_, ast) = parse_result.unwrap();
        let eval = math_utils::evaluate(&ast, 0.0);
        let expected = -1.0;
        assert!((eval - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn parentheses() {
        let expression = "2 * (4 - 2)";
        let parse_result = math_utils::parse_expr(expression);
        if let Err(e) = &parse_result {
            eprint!("{}", e);
            assert!(!parse_result.is_err());
        }
        
        let (_, ast) = parse_result.unwrap();
        let eval = math_utils::evaluate(&ast, 0.0);
        let expected = 4.0;
        assert!((eval - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn negative() {
        let expression = "-2 * (4 - 2)";
        let parse_result = math_utils::parse_expr(expression);
        if let Err(e) = &parse_result {
            eprint!("{}", e);
            assert!(!parse_result.is_err());
        }
        
        let (_, ast) = parse_result.unwrap();
        let eval = math_utils::evaluate(&ast, 0.0);
        let expected = -4.0;
        assert!((eval - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn modulo() {
        let expression = "7 % 3";
        let parse_result = math_utils::parse_expr(expression);
        if let Err(e) = &parse_result {
            eprint!("{}", e);
            assert!(!parse_result.is_err());
        }
        
        let (_, ast) = parse_result.unwrap();
        let eval = math_utils::evaluate(&ast, 0.0);
        let expected = 1.0;
        assert!((eval - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn formula() {
        let expression = "e ^ (pi / 2)";
        let parse_result = math_utils::parse_expr(expression);
        if let Err(e) = &parse_result {
            eprint!("{}", e);
            assert!(!parse_result.is_err());
        }
        
        let (_, ast) = parse_result.unwrap();
        let eval = math_utils::evaluate(&ast, 0.0);
        let expected = 4.810477380965351;
        println!("{}", eval);
        assert!((eval - expected).abs() < f64::EPSILON);
    }
}
