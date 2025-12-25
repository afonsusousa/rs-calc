use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i32),
    BinaryOp {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::BinaryOp { left, op, right } => write!(f, "({} {} {})", left, op, right),
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    fn new(message: &str) -> ParseError {
        ParseError { message: message.to_string() }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }
}

impl Error for ParseError {}

impl Expr {
    pub fn parse(input: &str) -> Result<Expr, ParseError> {
        let mut input = input;
        Expr::parse_addsub(&mut input)
    }

    pub fn eval(&self) -> i32 {
        match self {
            Expr::Number(n) => *n,
            Expr::BinaryOp { left, op, right } => {
                let l = left.eval();
                let r = right.eval();
                match op {
                    Op::Add => l + r,
                    Op::Sub => l - r,
                    Op::Mul => l * r,
                    Op::Div => l / r,
                }
            }
        }
    }

    fn consume(input: &mut &str, expected: &str) -> bool {
        let trimmed = input.trim_start();
        if trimmed.starts_with(expected) {
            *input = &trimmed[expected.len()..];
            true
        } else {
            false
        }
    }

    fn consume_number(input: &mut &str) -> Option<i32> {
        let trimmed = input.trim_start();

        let (is_negative, digits_start) = if trimmed.starts_with('-') {
            (true, 1)
        } else {
            (false, 0)
        };

        let len = trimmed[digits_start..]
            .find(|c: char| !c.is_digit(10))
            .map(|i| i + digits_start)
            .unwrap_or(trimmed.len());

        if len == digits_start {
            return None;
        }

        let (num_str, rest) = trimmed.split_at(len);
        *input = rest;
        num_str.parse::<i32>().ok()
    }

    fn parse_addsub(input: &mut &str) -> Result<Expr, ParseError> {
        let mut left = Expr::parse_multdiv(input)?;

        loop {
            let negative = Expr::consume(input, "-");
            if negative || Expr::consume(input, "+") {
                let right = Expr::parse_multdiv(input)?;
                let op = if negative { Op::Sub } else { Op::Add };

                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_multdiv(input: &mut &str) -> Result<Expr, ParseError> {
        let mut left = Expr::parse_brackets(input)?;

        loop {
            if Expr::consume(input, "*") {
                let right = Expr::parse_brackets(input)?;
                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op: Op::Mul,
                    right: Box::new(right),
                };
            } else if Expr::consume(input, "/") {
                let right = Expr::parse_brackets(input)?;
                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op: Op::Div,
                    right: Box::new(right),
                };
            } else {
                let trimmed = input.trim_start();
                if trimmed.starts_with('(') {
                    let right = Expr::parse_brackets(input)?;
                    left = Expr::BinaryOp {
                        left: Box::new(left),
                        op: Op::Mul,
                        right: Box::new(right),
                    };
                } else {
                    break;
                }
            }
        }
        Ok(left)
    }

    fn parse_brackets(input: &mut &str) -> Result<Expr, ParseError> {
        if Expr::consume(input, "(") {
            let expr = Expr::parse_addsub(input)?;
            if Expr::consume(input, ")") {
                Ok(expr)
            } else {
                Err(ParseError::new("Mismatched parenthesis"))
            }
        } else {
            if let Some(num) = Expr::consume_number(input) {
                Ok(Expr::Number(num))
            } else {
                Err(ParseError::new("Expected number or '('"))
            }
        }
    }
}

fn main() {
    let data = "2(3 - 1)";
    match Expr::parse(data) {
        Ok(tree) => println!("Implicit multiplication (2(3 - 1)): {} => {}", tree, tree.eval()),
        Err(e) => println!("Error: {}", e),
    }

    let data2 = "10 + 2 * (3 - 1)";
    match Expr::parse(data2) {
        Ok(tree) => println!("Standard (10 + 2 * (3 - 1)): {} => {}", tree, tree.eval()),
        Err(e) => println!("Error: {}", e),
    }
}
