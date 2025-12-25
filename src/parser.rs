use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::expr::Expr;
use crate::op::Op;

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

pub struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { input }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.parse_addsub()
    }

    fn peek(&self) -> Option<char> {
        self.input.trim_start().chars().next()
    }

    fn consume(&mut self, expected: &str) -> bool {
        let trimmed = self.input.trim_start();
        if trimmed.starts_with(expected) {
            self.input = &trimmed[expected.len()..];
            true
        } else {
            false
        }
    }

    fn consume_number(&mut self) -> Option<f64> {
        let trimmed = self.input.trim_start();

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
        self.input = rest;
        num_str.parse::<f64>().ok()
    }

    fn parse_addsub(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_multdiv()?;

        loop {
            if self.consume("+") {
                let right = self.parse_multdiv()?;
                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op: Op::Add,
                    right: Box::new(right),
                };
            } else if self.consume("-") {
                let right = self.parse_multdiv()?;
                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op: Op::Sub,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_multdiv(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_brackets()?;

        loop {
            if self.consume("*") {
                let right = self.parse_brackets()?;
                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op: Op::Mul,
                    right: Box::new(right),
                };
            } else if self.consume("/") {
                let right = self.parse_brackets()?;
                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op: Op::Div,
                    right: Box::new(right),
                };
            } else {
                if let Some('(') = self.peek() {
                    let right = self.parse_brackets()?;
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

    fn parse_brackets(&mut self) -> Result<Expr, ParseError> {
        if self.consume("(") {
            let expr = self.parse_addsub()?;
            if self.consume(")") {
                Ok(expr)
            } else {
                Err(ParseError::new("Mismatched parenthesis"))
            }
        } else {
            if let Some(num) = self.consume_number() {
                Ok(Expr::Number(num))
            } else {
                Err(ParseError::new("Expected number or '('"))
            }
        }
    }
}

