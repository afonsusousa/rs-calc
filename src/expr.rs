use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::op::Op;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
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
pub enum EvalError {
    DivisionByZero,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::DivisionByZero => write!(f, "Division by zero"),
        }
    }
}

impl Error for EvalError {}

impl Expr {
    pub fn eval(&self) -> Result<f64, EvalError> {
        match self {
            Expr::Number(n) => Ok(*n),
            Expr::BinaryOp { left, op, right } => {
                let l = left.eval()?;
                let r = right.eval()?;
                match op {
                    Op::Add => Ok(l + r),
                    Op::Sub => Ok(l - r),
                    Op::Mul => Ok(l * r),
                    Op::Div => {
                        if r == 0.into() {
                            Err(EvalError::DivisionByZero)
                        } else {
                            Ok(l / r)
                        }
                    },
                }
            }
        }
    }
}