#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    fn evaluate(input: &str) -> Result<f64, String> {
        let mut parser = Parser::new(input);
        match parser.parse() {
            Ok(expr) => expr.eval().map_err(|e| e.to_string()),
            Err(e) => Err(e.to_string()),
        }
    }

    #[test]
    fn test_simple_addition() {
        assert_eq!(evaluate("1 + 2"), Ok(3.0));
    }

    #[test]
    fn test_simple_subtraction() {
        assert_eq!(evaluate("5 - 2"), Ok(3.0));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(evaluate("3 * 4"), Ok(12.0));
    }

    #[test]
    fn test_division() {
        assert_eq!(evaluate("10 / 2"), Ok(5.0));
    }

    #[test]
    fn test_precedence() {
        assert_eq!(evaluate("1 + 2 * 3"), Ok(7.0));
        assert_eq!(evaluate("10 - 2 * 3"), Ok(4.0));
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(evaluate("(1 + 2) * 3"), Ok(9.0));
        assert_eq!(evaluate("10 / (2 + 3)"), Ok(2.0));
    }

    #[test]
    fn test_nested_parentheses() {
        assert_eq!(evaluate("((1 + 2) * (3 + 4))"), Ok(21.0));
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(evaluate("-5 + 3"), Ok(-2.0));
        assert_eq!(evaluate("10 + -2"), Ok(8.0));
        assert_eq!(evaluate("-5 * -2"), Ok(10.0));
    }

    #[test]
    fn test_implicit_multiplication() {
        assert_eq!(evaluate("2(3)"), Ok(6.0));
        assert_eq!(evaluate("2(3 + 1)"), Ok(8.0));
        assert_eq!(evaluate("(2)(3)"), Ok(6.0));
        assert_eq!(evaluate("-2(-3(1/3))"), Ok(2.0));
    }

    #[test]
    fn test_left_associativity() {
        assert_eq!(evaluate("10 - 2 - 3"), Ok(5.0)); // (10 - 2) - 3 = 5
        assert_eq!(evaluate("10 / 2 / 5"), Ok(1.0)); // (10 / 2) / 5 = 1
    }

    #[test]
    fn test_division_by_zero() {
        assert!(evaluate("1 / 0").is_err());
        assert!(evaluate("1 / (5 - 5)").is_err());
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(evaluate("  1   +   2  "), Ok(3.0));
    }
}